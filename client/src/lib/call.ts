import { BASE_API_URL } from ".";

type Method = "GET" | "POST" | "PUT" | "DELETE";

export default async function call(endpoint: string, method: Method) {
  return await fetch(`${BASE_API_URL}${endpoint}`, fetchOptions(method));
}

export function fetchOptions(method: Method): RequestInit {
  return {
    method,
    credentials: "include",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
  };
}
