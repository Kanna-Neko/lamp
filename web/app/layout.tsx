import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "lamp",
  description: "An api virtualization tool.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        {children}
      </body>
    </html>
  );
}
