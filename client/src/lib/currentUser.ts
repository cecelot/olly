import { Member } from "~/types";

export const currentUser = async () => {
  const res = await fetch("http://localhost:3000/@me", {
    credentials: "include",
    mode: "cors",
  });
  if (res.status === 200) {
    const member: { message: Member } = await res.json();
    return member.message;
  }
  return null;
};
