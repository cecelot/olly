import { BASE_API_URL } from ".";

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
    alert("An unexpected error occurred!");
  }
};
