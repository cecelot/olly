import showToast from "~/lib/showToast";

export const createGame = async (opponent: string) => {
  const res = await fetch("http://localhost:3000/game", {
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
    showToast(
      {
        404: {
          text: "That user doesn't exist! Make sure their username is spelled correctly.",
          kind: "error",
        },
        401: { text: "You must be logged in to create a game.", kind: "error" },
      },
      res.status
    );
  }
};
