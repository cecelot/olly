import call from "@/lib/call";

export default async function simpleGet(endpoint: string) {
  const resp = await call(endpoint, "GET");
  const { message } = await resp.json();
  return message;
}
