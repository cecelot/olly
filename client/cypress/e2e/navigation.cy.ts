describe("Navigation", () => {
  it("should navigate to the about page", () => {
    cy.visit("/");
    cy.get('a[href*="about"]').click();
    cy.url().should("include", "/about");
    cy.get("p").contains(
      "Olly is a free and open source game client and server",
    );
  });

  it("should navigate to the home page", () => {
    cy.visit("/");
    cy.get("h1").contains("Othello");
    cy.get("h2").contains("Play online with friends or against the computer!");
  });
});
