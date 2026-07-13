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

export namespace ResettableFlatPromise {}

export interface ResettableFlatPromise<Type = void> extends FlatPromise.Resolver<Type> {
  promise: PromiseLike<Type>;
  reset: () => void;
}

export function resettableFlatPromise<Type = void>(): ResettableFlatPromise<Type> {
  let state: "pending" | "resolved" | "rejected" = "pending";
  let value: Type;
  let reason: unknown;
  let waiters: FlatPromise.Resolver<Type>[] = [];

  const currentPromise = () =>
    new Promise<Type>((resolve, reject) => {
      if (state === "resolved") return resolve(value);
      if (state === "rejected") return reject(reason);
      waiters.push({ resolve, reject });
    });

  const promise: PromiseLike<Type> = {
    then: (onfulfilled, onrejected) => currentPromise().then(onfulfilled, onrejected),
  };

  return {
    promise,

    resolve(nextValue) {
      state = "resolved";
      value = nextValue;
      const pendingWaiters = waiters;
      waiters = [];
      pendingWaiters.forEach((waiter) => waiter.resolve(nextValue));
    },

    reject(nextReason) {
      state = "rejected";
      reason = nextReason;
      const pendingWaiters = waiters;
      waiters = [];
      pendingWaiters.forEach((waiter) => waiter.reject(nextReason));
    },

    reset() {
      state = "pending";
    },
  };
}
