import { Input } from "@headlessui/react";
import { HTMLInputTypeAttribute } from "react";

interface FieldProps {
  type: HTMLInputTypeAttribute;
  placeholder: string;
  errorText?: string;
  setText: (value: string) => void;
}

export default function ErrorableFormInput({
  type,
  placeholder,
  errorText,
  setText,
}: FieldProps) {
  return (
    <>
      <Input
        type={type}
        placeholder={placeholder}
        className="bg-crust text-subtext0 rounded-lg p-3"
        onChange={(e) => setText(e.currentTarget.value)}
      ></Input>
      {errorText && (
        <p className="text-xs text-red">
          {errorText || "An unexpected error occurred."}
        </p>
      )}
    </>
  );
}
