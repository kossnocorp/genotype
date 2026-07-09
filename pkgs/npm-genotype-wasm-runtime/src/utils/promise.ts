/**
 * Flat promise namespace. It holds relevant types.
 */
export namespace FlatPromise {
  /**
   * The resolver type. It contains the resolve and reject functions.
   */
  export interface Resolver<Type = void> {
    /** The resolve function. */
    resolve: (value: Type) => void;
    /** The reject function. */
    reject: (reason?: unknown) => void;
  }
}

/**
 * Flat promise type. It's a promise with resolve and reject functions
 * as an object.
 */
export interface FlatPromise<Type = void> extends FlatPromise.Resolver<Type> {
  /** The promise. */
  promise: Promise<Type>;
}

/**
 * The function returns object with promise and the control functions. It allows
 * to pass resolve and reject functions as arguments.
 *
 * @returns The flat promise.
 */
export default function flatPromise<Type = void>(): FlatPromise<Type> {
  let resolve: (value: Type) => void;
  let reject: (reason?: unknown) => void;

  const promise = new Promise<Type>((res, rej) => {
    resolve = res;
    reject = rej;
  });

  return {
    promise,
    resolve: resolve!,
    reject: reject!,
  };
}
