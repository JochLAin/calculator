const symbolPropertyList = Symbol('§§nui::property-list');

export type PropertyKind = BooleanConstructor | NumberConstructor | StringConstructor;
export type PropertyType<K extends PropertyKind> =
  K extends BooleanConstructor
    ? boolean
    : K extends NumberConstructor
      ? number
      : string;

export type PropertyListener<E extends HTMLElement, T extends PropertyType<PropertyKind>> = <H extends E>(host: H, value: T | null, oldValue: T | null) => void;

export type Property<E extends HTMLElement, K extends PropertyKind> = {
  descriptor?: PropertyDescriptor,
  name: string,
  kind?: K,
  listener?: PropertyListener<E, PropertyType<K>>,
};

function decorateProperty<E extends HTMLElement, K extends PropertyKind>(target: E, name: string, { descriptor, kind, listener }: Property<E, K>) {
  // @ts-ignore
  const defaultValue = target[name];
  // @ts-ignore
  delete target[name];

  function get(): PropertyType<K> | null {
    const value = target.getAttribute(name);
    switch (kind) {
      case Boolean:
        // @ts-ignore
        if (!target.hasAttribute(name)) return false;
        // @ts-ignore
        return 'false' !== value;
      case Number:
        if (null === value) return null;
        // @ts-ignore
        return Number(value);
      default:
        // @ts-ignore
        return value;
    }
  }

  function set(value: PropertyType<K>|null) {
    const oldValue = get();
    if (value === null) {
      target.removeAttribute(name);
      listener?.call(target, target, value, oldValue);
    } else if (value !== oldValue) {
      if (Boolean === kind) {
        target.toggleAttribute(name, !!value);
      } else {
        target.setAttribute(name, String(value));
      }
      listener?.call(target, target, value, oldValue);
    }
  }

  Object.defineProperty(target, name, { configurable: false, enumerable: true, get, set });

  if (defaultValue !== undefined) {
    if (Boolean === kind) {
      // @ts-ignore
      set(!!defaultValue);
    } else if (Number === kind) {
      // @ts-ignore
      set(Number(defaultValue));
    } else {
      set(defaultValue);
    }
  }
}

function parsePropertyDecoratorParameters<E extends HTMLElement, K extends PropertyKind>(kindParam?: PropertyKind|PropertyListener<E, string>, listenerParam?: PropertyListener<E, PropertyType<K>>): [K, undefined|PropertyListener<E, PropertyType<K>>] {
  let kind: K|null = null;
  let listener: PropertyListener<E, PropertyType<K>> | undefined = undefined;

  if (kindParam === Boolean || kindParam === Number || kindParam === String) {
    kind = kindParam as K;
    listener = listenerParam;
  } else if (typeof kindParam === 'function') {
    // @ts-ignore
    listener = kindParam;
  }

  if (!kind) {
    kind = String as K;
  }

  return [kind, listener];
}

export function property<E extends HTMLElement, K extends PropertyKind>(kind: K, listener?: PropertyListener<E, PropertyType<K>>): PropertyDecorator|MethodDecorator;
export function property<E extends HTMLElement>(listener?: PropertyListener<E, string>): PropertyDecorator|MethodDecorator;
export function property<E extends HTMLElement, K extends PropertyKind>(kindParam?: K|PropertyListener<E, string>, listenerParam?: PropertyListener<E, PropertyType<K>>): PropertyDecorator|MethodDecorator {
  let [kind, listener] = parsePropertyDecoratorParameters<E, K>(kindParam, listenerParam);

  // @ts-ignore
  return function decorate(target: E, name: string, descriptor?: PropertyDescriptor) {
    if (!target.constructor.hasOwnProperty(symbolPropertyList)) {
      const mapProperties = new Map<string, Property<HTMLElement, any>>();
      Object.defineProperty(target.constructor, symbolPropertyList, {
        enumerable: false,
        get: () => mapProperties,
        set: () => true,
      });
    }

    // @ts-ignore
    target.constructor.observedAttributes = [target.constructor.observedAttributes || [], name];
    // @ts-ignore
    target.constructor[symbolPropertyList].set(name, { descriptor, kind, listener });
  }
}

export function element<E extends typeof HTMLElement, H>(tag: string, template?: HTMLTemplateElement, handle?: <T extends InstanceType<E>>(target: T, shadow: ShadowRoot|null) => H) {
  return function (targetClass: E) {
    const name = `HTMLNui${targetClass.name.replace(/^(HTML)/, '')}`;
    // @ts-ignore
    const DecoratedClass = class extends targetClass {
      constructor() {
        super();

        const target = this as InstanceType<E>;
        // @ts-ignore
        this.constructor[symbolPropertyList]?.forEach((prop, name) => {
          // @ts-ignore
          decorateProperty<InstanceType<E>>(target, name, prop);
        });

        let shadow: ShadowRoot|null = null;
        if (template) {
          shadow = target.attachShadow({ mode: 'closed' });
          shadow.appendChild(template.content.cloneNode(true));
        }
      }

      public attributeChangedCallback(name: string, oldValue: string, newValue: string) {
        // @ts-ignore
        targetClass.prototype.attributeChangedCallback?.call(this, name, oldValue, newValue);
        // @ts-ignore
        if (this.constructor[symbolPropertyList]?.has(name)) {
          // @ts-ignore
          const { listener } = this.constructor[symbolPropertyList].get(name);
          // @ts-ignore
          listener?.call<InstanceType<E>>(this, this, newValue, oldValue);
        }
      }
    }

    Object.defineProperty(DecoratedClass, 'name', { value: name });
    customElements.define(tag, DecoratedClass as CustomElementConstructor)
    return DecoratedClass;
  }
}
