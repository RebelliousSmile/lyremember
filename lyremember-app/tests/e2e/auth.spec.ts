describe('Authentication', () => {
  it('should show login page on start', async () => {
    const title = await $('h2');
    await title.waitForExist({ timeout: 5000 });
    const text = await title.getText();
    expect(text).toContain('Connexion');
  });

  it('should navigate to register page', async () => {
    const registerLink = await $('a[href="#/register"]');
    await registerLink.click();

    const title = await $('h2');
    await title.waitForExist({ timeout: 5000 });
    const text = await title.getText();
    expect(text).toContain('Créer');
  });

  it('should register a new user', async () => {
    const usernameInput = await $('input[type="text"]');
    await usernameInput.setValue('testuser');

    const emailInput = await $('input[type="email"]');
    await emailInput.setValue('test@example.com');

    const passwordInputs = await $$('input[type="password"]');
    await passwordInputs[0].setValue('password123');
    await passwordInputs[1].setValue('password123');

    const submitBtn = await $('button[type="submit"]');
    await submitBtn.click();

    // Should redirect to dashboard
    await browser.waitUntil(
      async () => (await browser.getUrl()).includes('dashboard'),
      { timeout: 10000, timeoutMsg: 'Expected to navigate to dashboard' }
    );
  });

  it('should login as guest', async () => {
    // Navigate back to login
    await browser.url('#/login');

    // Click "Continuer sans compte"
    const buttons = await $$('button');
    let guestBtn: WebdriverIO.Element | undefined;
    for (const btn of buttons) {
      const text = await btn.getText();
      if (text.includes('sans compte') || text.includes('without')) {
        guestBtn = btn;
        break;
      }
    }
    expect(guestBtn).toBeDefined();
    await guestBtn!.click();

    // Should redirect to dashboard
    await browser.waitUntil(
      async () => (await browser.getUrl()).includes('dashboard'),
      { timeout: 10000, timeoutMsg: 'Expected to navigate to dashboard' }
    );
  });
});
