export type Token = {
  name?: string,
  issuer?: string,
  placeholder: string,
  otp?: string,
  label_input_name?: string,
}

const Tokens = {
  new: (token: Token[]): Token[] => token,
}

export default Tokens;
