export namespace Wc {
  export type LocateParent = Element | HTMLElement | DocumentFragment;

  export type ElementClass<Type extends HTMLElement> = abstract new (...args: any[]) => Type;

  export type IterateCallback<Type extends HTMLElement> = (el: Type) => void;
}

export class Wc extends HTMLElement {
  //#region locate

  static locate(parentEl: Wc.LocateParent, selector: string): HTMLElement;

  static locate(parentEl: Wc.LocateParent, selector: string, ElementClass: undefined): HTMLElement;

  static locate<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): Type;

  static locate<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    ElementClass?: Wc.ElementClass<Type> | undefined,
  ): Type | HTMLElement {
    const el = parentEl.querySelector(selector);
    Wc.#ensure(selector, el, ElementClass);
    return el;
  }

  locate(selector: string): HTMLElement;

  locate(selector: string, ElementClass: undefined): HTMLElement;

  locate<Type extends HTMLElement>(
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): Type;

  locate<Type extends HTMLElement>(
    selector: string,
    ElementClass?: Wc.ElementClass<Type> | undefined,
  ): Type | HTMLElement {
    return Wc.locate(this, selector, ElementClass);
  }

  //#endregion

  //#region locateAll

  static locateAll(parentEl: Wc.LocateParent, selector: string): HTMLElement[];

  static locateAll<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): Type[];

  static locateAll<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    ElementClass?: Wc.ElementClass<Type> | undefined,
  ): Array<Type | HTMLElement> {
    const els = Array.from(parentEl.querySelectorAll(selector));
    return els.map((el) => {
      Wc.#ensure(selector, el, ElementClass);
      return el;
    });
  }

  locateAll(selector: string): HTMLElement[];

  locateAll<Type extends HTMLElement>(
    selector: string,
    ElementClass: Wc.ElementClass<Type>,
  ): Type[];

  locateAll<Type extends HTMLElement>(
    selector: string,
    ElementClass?: Wc.ElementClass<Type>,
  ): Array<Type | HTMLElement> {
    return Wc.locateAll(this, selector, ElementClass);
  }

  //#endregion

  //#region locateClosest

  static locateClosest(parentEl: Element, selector: string): HTMLElement;

  static locateClosest(parentEl: Element, selector: string, ElementClass: undefined): HTMLElement;

  static locateClosest<Type extends HTMLElement>(
    parentEl: Element,
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): Type;

  static locateClosest<Type extends HTMLElement>(
    parentEl: Element,
    selector: string,
    ElementClass?: Wc.ElementClass<Type> | undefined,
  ): Type | HTMLElement {
    const closestEl = parentEl.closest(selector);
    Wc.#ensure(selector, closestEl, ElementClass);
    return closestEl;
  }

  locateClosest(selector: string): HTMLElement;

  locateClosest(selector: string, ElementClass: undefined): HTMLElement;

  locateClosest<Type extends HTMLElement>(
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): Type;

  locateClosest<Type extends HTMLElement>(
    selector: string,
    ElementClass?: Wc.ElementClass<Type> | undefined,
  ): Type | HTMLElement {
    return Wc.locateClosest(this, selector, ElementClass);
  }

  //#endregion

  //#region locateEach

  static locateEach(
    parentEl: Wc.LocateParent,
    selector: string,
    callback: Wc.IterateCallback<HTMLElement>,
  ): void;

  static locateEach<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
    callback: Wc.IterateCallback<Type>,
  ): void;

  static locateEach<Type extends HTMLElement>(
    parentEl: Wc.LocateParent,
    selector: string,
    callbackOrElementClass: Wc.ElementClass<Type> | Wc.IterateCallback<Type> | undefined,
    maybeCallback?: Wc.IterateCallback<Type>,
  ): void {
    const els = Array.from(parentEl.querySelectorAll(selector));

    const ElementClass = maybeCallback
      ? (callbackOrElementClass as Wc.ElementClass<Type> | undefined)
      : undefined;
    const callback = maybeCallback || (callbackOrElementClass as Wc.IterateCallback<Type>);

    return els.forEach((el) => {
      Wc.#ensure(selector, el, ElementClass);
      callback(el);
      return el;
    });
  }

  each(selector: string, callback: Wc.IterateCallback<HTMLElement>): void;

  each<Type extends HTMLElement>(
    selector: string,
    ElementClass: Wc.ElementClass<Type> | undefined,
    callback: Wc.IterateCallback<Type>,
  ): void;

  each<Type extends HTMLElement>(
    selector: string,
    callbackOrElementClass: Wc.ElementClass<Type> | Wc.IterateCallback<Type> | undefined,
    maybeCallback?: Wc.IterateCallback<Type>,
  ): void {
    if (maybeCallback) {
      Wc.locateEach(
        this,
        selector,
        callbackOrElementClass as Wc.ElementClass<Type> | undefined,
        maybeCallback,
      );
    } else {
      Wc.locateEach(this, selector, callbackOrElementClass as Wc.IterateCallback<HTMLElement>);
    }
  }

  //#endregion

  //#region Internals

  static #ensure<Type extends HTMLElement>(
    selector: string,
    el: Element | null,
    ElementClass: Wc.ElementClass<Type> | undefined,
  ): asserts el is Type {
    const Class = ElementClass || HTMLElement;
    if (!el) throw new Error(`Selector "${selector}" not found`);
    if (!(el instanceof Class))
      throw new Error(`Selector "${selector}" result is not an instance of \`${Class.name}\``);
  }

  //#endregion
}
