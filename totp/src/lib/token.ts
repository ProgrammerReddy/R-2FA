export type Token = {
  id: number,
  placeholder: string,
  issuer: string,
  otp: string,
};

export type StructToken = {
  id: number,
  account_name: string,
  issuer: string,
  secret: string,
};
