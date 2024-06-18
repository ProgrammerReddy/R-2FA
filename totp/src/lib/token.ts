export type Token = {
  issuer: string,
  icon: string,
  otp: string,
}

const Tokens = {
  new: (token: Token[]): Token[] => token,
}

export default Tokens;
