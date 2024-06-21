import toast from "react-hot-toast";
import { BASE_API_URL, TOAST_ERROR_OPTIONS, TOAST_SUCCESS_OPTIONS } from ".";

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
  const { message } = await res.json();
  if (res.status === 201) {
    toast.success(
      `Sent invite to ${opponent}! You'll be able to join the game from the home page if they accept.`,
      TOAST_SUCCESS_OPTIONS
    );
    setTimeout(() => {
      window.location.href = "/";
    }, 3000);
  } else {
    toast.error(message, TOAST_ERROR_OPTIONS);
  }
};
