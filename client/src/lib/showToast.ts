import toast from "solid-toast";

interface Message {
  text: string;
  kind: "success" | "error";
  afterClose?: () => void;
}

export default function showToast(
  messages: Record<number, Message>,
  code: number
) {
  const message = messages[code];
  if (message) {
    toast[message.kind](message.text);
    if (message.afterClose) {
      setTimeout(message.afterClose, 3000);
    }
  } else {
    toast.error("An unexpected error occurred. Try again later.");
  }
}
