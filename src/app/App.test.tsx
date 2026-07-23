import { render, screen } from "@testing-library/react";
import { App } from "./App";

test("renders the Pavo application shell", () => {
  render(<App />);
  expect(screen.getByRole("application", { name: "Pavo" })).toBeInTheDocument();
});
