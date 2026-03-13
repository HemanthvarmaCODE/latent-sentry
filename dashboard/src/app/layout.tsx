import "./globals.css";

export const metadata = {
  title: "Latent-Sentry Dashboard",
  description: "CLBD Monitor"
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}