describe("Register", () => {
  it("should successfully register a new user and redirect to the home page", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("password1");
    cy.get("button").click();
    cy.url().should("equal", "http://localhost:8000/");
  });

  it("should display an error message when the username is already taken", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("password1");
    cy.get("button").click();
    cy.get("#username-error").contains("Username is already taken!");
  });

  it("should display an error message when the username is too short", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("a");
    cy.get("input[name=password]").type("password1");
    cy.get("button").click();
    cy.get("#username-error").contains(
      "Username must be at least 3 characters.",
    );
  });

  it("should display an error message when the password is too short", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("pass");
    cy.get("button").click();
    cy.get("#password-error").contains(
      "Password must be at least 8 characters.",
    );
  });

  it("should display an error message when the password does not contain a number", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("password");
    cy.get("button").click();
    cy.get("#password-error").contains(
      "Password must contain at least one number.",
    );
  });

  it("should display an error message when the password does not contain a letter", () => {
    cy.visit("/register");
    cy.get("h1").contains("Register");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("12345678");
    cy.get("button").click();
    cy.get("#password-error").contains(
      "Password must contain at least one alphabetic character.",
    );
  });
});

describe("Login", () => {
  it("should successfully login a user and redirect to the home page", () => {
    cy.visit("/login");
    cy.get("h1").contains("Login");
    cy.get("input[name=username]").type("testuser");
    cy.get("input[name=password]").type("password1");
    cy.get("button").click();
    cy.url().should("equal", "http://localhost:8000/");
  });
});
