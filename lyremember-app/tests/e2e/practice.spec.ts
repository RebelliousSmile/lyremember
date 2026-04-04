describe('Practice - Mémoriser', () => {
  before(async () => {
    // Login as guest
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

  it('should navigate to practice page', async () => {
    await browser.url('#/practice');
    const body = await $('body');
    await body.waitForExist({ timeout: 5000 });
    const text = await body.getText();
    expect(text.toLowerCase()).toContain('mémoriser');
  });

  it('should show stats overview', async () => {
    const body = await $('body');
    const text = await body.getText();
    // Should show stats even if zero
    expect(text).toMatch(/\d/);
  });

  it('should navigate to song detail and start practice mode', async () => {
    // First create a song to practice
    await browser.url('#/songs/add');

    const titleInput = await $('input[required]');
    await titleInput.setValue('Test Song');

    const inputs = await $$('input');
    await inputs[1].setValue('Test Artist');

    const langSelect = await $('select');
    await langSelect.selectByAttribute('value', 'fr');

    const textarea = await $('textarea');
    await textarea.setValue('Ligne un\nLigne deux\nLigne trois');

    const submitBtn = await $('button[type="submit"]');
    await submitBtn.click();

    await browser.waitUntil(
      async () => {
        const url = await browser.getUrl();
        return url.includes('songs/') && !url.includes('add');
      },
      { timeout: 15000 }
    );

    // Check that practice mode buttons are visible
    const body = await $('body');
    const text = await body.getText();
    const hasPractice = text.includes('Karaoké') || text.includes('Karaoke') || text.includes('trous');
    expect(hasPractice).toBe(true);
  });
});
