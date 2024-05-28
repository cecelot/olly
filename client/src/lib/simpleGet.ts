import { call } from ".";

export const simpleGet = async (endpoint: string) => {
  const resp = await call(endpoint, "GET");
  const { message } = await resp.json();
  return message;
};
