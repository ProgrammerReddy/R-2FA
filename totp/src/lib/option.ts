type None = "unknown token";
export type Some<T> = { value: T };
type Option<T> = None | Some<T>;

export const none: None = "unknown token";
export const some = <T>(x: T): Some<T> => ({ value: x });
export const option = <T>(x: Promise<T>): Promise<Option<T>> => 
  x.then((resolve: T): Some<T> => some(resolve)).catch((): None => none);
