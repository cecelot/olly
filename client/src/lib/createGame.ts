import toast from "react-hot-toast";
import { BASE_API_URL, TOAST_ERROR_OPTIONS } from ".";

export const createGame = async (opponent: string) => {
  const res = await fetch(`${BASE_API_URL}/game`, {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      guest: opponent,
    }),
  });
  if (res.status === 201) {
    const { message } = await res.json();
    window.location.href = `/play?gameId=${message.id}`;
  } else {
    const { message } = await res.json();
    toast.error(message, TOAST_ERROR_OPTIONS);
  }
};
