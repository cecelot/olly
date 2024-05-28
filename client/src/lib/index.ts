export { createGame } from "./createGame";
export { currentUser } from "./currentUser";
export { simpleGet } from "./simpleGet";

type Method = "GET" | "POST" | "PUT" | "DELETE";

export const BASE_URL = "http://localhost:3000";

export const call = async (endpoint: string, method: Method) => {
  return await fetch(`${BASE_URL}${endpoint}`, fetchOptions(method));
};

export const fetchOptions = (method: Method): RequestInit => {
  return {
    method,
    credentials: "include",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
  };
};
