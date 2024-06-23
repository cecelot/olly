import { Input } from "@headlessui/react";
import { HTMLInputTypeAttribute } from "react";

interface FieldProps {
  name: string;
  type: HTMLInputTypeAttribute;
  placeholder: string;
  errorText?: string;
  setText: (value: string) => void;
}

export default function ErrorableFormInput({
  name,
  type,
  placeholder,
  errorText,
  setText,
}: FieldProps) {
  return (
    <>
      <Input
        name={name}
        type={type}
        placeholder={placeholder}
        className="bg-crust text-subtext0 rounded-lg p-3"
        onChange={(e) => setText(e.currentTarget.value)}
      ></Input>
      {errorText && (
        <p id={`${name}-error`} className="text-xs text-red">
          {errorText || "An unexpected error occurred."}
        </p>
      )}
    </>
  );
}
