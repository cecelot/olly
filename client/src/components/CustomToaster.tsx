import { Toaster } from "solid-toast";

export default function CustomToaster() {
  return (
    <Toaster
      toastOptions={{
        duration: 3000,
        unmountDelay: 500,
        position: "top-center",
        iconTheme: {
          primary: "#ea76cb",
        },
        style: {
          "background-color": "#e6e9ef",
        },
      }}
    />
  );
}
