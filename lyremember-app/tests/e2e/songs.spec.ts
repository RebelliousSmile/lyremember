describe('Songs - Ma Lyre', () => {
  before(async () => {
    // Login as guest first
    const buttons = await $$('button');
    for (const btn of buttons) {
      const text = await btn.getText();
      if (text.includes('sans compte') || text.includes('without')) {
        await btn.click();
        break;
      }
    }
    await browser.waitUntil(
      async () => (await browser.getUrl()).includes('dashboard'),
      { timeout: 10000 }
    );
  });

  it('should navigate to Ma Lyre (songs list)', async () => {
    await browser.url('#/songs');
    const header = await $('h1');
    await header.waitForExist({ timeout: 5000 });
    const text = await header.getText();
    expect(text).toContain('Lyre');
  });

  it('should show empty state when no songs', async () => {
    const emptyText = await $('p');
    const text = await emptyText.getText();
    expect(text.length).toBeGreaterThan(0);
  });

  it('should navigate to add song page', async () => {
    await browser.url('#/songs/add');
    const header = await $('h1');
    await header.waitForExist({ timeout: 5000 });
    const text = await header.getText();
    expect(text.toLowerCase()).toContain('ajouter');
  });

  it('should create a song', async () => {
    // Fill title
    const titleInput = await $('input[required]');
    await titleInput.setValue('Bohemian Rhapsody');

    // Fill artist
    const inputs = await $$('input');
    await inputs[1].setValue('Queen');

    // Select language
    const langSelect = await $('select');
    await langSelect.selectByAttribute('value', 'en');

    // Fill lyrics
    const textarea = await $('textarea');
    await textarea.setValue('Is this the real life\nIs this just fantasy');

    // Submit
    const submitBtn = await $('button[type="submit"]');
    await submitBtn.click();

    // Should navigate back to songs or song detail
    await browser.waitUntil(
      async () => {
        const url = await browser.getUrl();
        return url.includes('songs') && !url.includes('add');
      },
      { timeout: 15000, timeoutMsg: 'Expected to navigate after song creation' }
    );
  });

  it('should display the created song in the list', async () => {
    await browser.url('#/songs');
    await browser.pause(1000);

    const pageText = await $('body').getText();
    expect(pageText).toContain('Bohemian Rhapsody');
    expect(pageText).toContain('Queen');
  });
});
