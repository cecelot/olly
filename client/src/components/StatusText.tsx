interface StatusTextProps {
  text: string;
}

export default function StatusText({ text }: StatusTextProps) {
  return (
    <main className="flex justify-center text-center">
      <section className="flex flex-col max-h-screen items-center p-5">
        <h1 className="text-text">{text}</h1>
      </section>
    </main>
  );
}
