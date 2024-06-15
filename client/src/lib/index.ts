import { ToastOptions } from "react-hot-toast";

export const BASE_API_URL = "http://localhost:3000";

const TOAST_BASE_OPTIONS: ToastOptions = {
  position: "bottom-center",
  duration: 3000,
  style: {
    backgroundColor: "#e6e9ef",
  },
};

export const TOAST_SUCCESS_OPTIONS: ToastOptions = {
  ...TOAST_BASE_OPTIONS,
  iconTheme: {
    primary: "#40a02b",
    secondary: "#eff1f5",
  },
};

export const TOAST_ERROR_OPTIONS: ToastOptions = {
  ...TOAST_BASE_OPTIONS,
  iconTheme: {
    primary: "#d20f39",
    secondary: "#eff1f5",
  },
};
