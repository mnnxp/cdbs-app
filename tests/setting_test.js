import { Selector } from 'testcafe'; // first import testcafe selectors

const apiPort = process.env.PORT || 8080;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

const programTarget = 'Creo';
const programTarget2 = 'DesignSpark Mechanical';
const programSelect = Selector('#program');
const programOption = programSelect.find('option');

const regionTarget = 'Chuy valley';
const regionTarget2 = 'Talas region';
const regionSelect = Selector('#region');
const regionOption = regionSelect.find('option');

const registerButton = Selector('a').withAttribute('href', '#/register');
const settingButton = Selector('a').withAttribute('href', '#/settings');

const goodFirstname = "Testfirstname";
const goodLastname = "Testlastname";
const goodSecondname = "Testsecondname";
const goodUsername = "testusername"+Math.random();
const goodPassword = "testpassword";
const goodEmail = "test@example.domen";
const goodDescription = "description for test username";
const goodPosition = "engineer";
const goodPhone = "+321234567890";
const goodAddress = "City, Street, home, appart 1";
const goodProgram = "Creo";
const goodRegion = "Altai Republic";

fixture `Check update user data`// declare the fixture
    .page `${url}`;  // specify the start page

// Update user data and return old data
test('Update user data', async t => {
  // open register page
  await t.click(registerButton)

  await t
      .typeText('#firstname', goodFirstname)
      .typeText('#lastname', goodLastname)
      .typeText('#secondname', goodSecondname)
      .typeText('#email', goodEmail)
      .typeText('#username', goodUsername)
      .typeText('#password', goodPassword)
      .click(programSelect)
      .click(programOption.withText(goodProgram))
      .click(regionSelect)
      .click(regionOption.withText(goodRegion))
      .click('#submit-button')

    await t
        .typeText('#username', goodUsername)
        .typeText('#password', goodPassword)
        .click('#submit-button')

        // check route to home
        .expect(Selector('h2').filter('.subtitle').innerText).eql('engineer component supplier');

    // open setting page
    await t.click(settingButton)

    await t
        .typeText('#firstname', goodFirstname)
        .typeText('#lastname', goodLastname)
        .typeText('#secondname', goodSecondname)
        // .typeText('#username', goodUsername)
        .typeText('#email', goodEmail, { replace: true })
        .typeText('#description', goodDescription)
        .typeText('#position', goodPosition)
        .typeText('#phone', goodPhone)
        .typeText('#address', goodAddress)
        .click(programSelect)
        .click(programOption.withText(programTarget))
        .click(regionSelect)
        .click(regionOption.withText(regionTarget))
        .click('#update-settings')

        // check count updated rows
        .expect(Selector('span')
          .filter('.tag')
          .filter('.is-info')
          .filter('.is-light')
          .innerText).eql('Updated rows: 10');

    await t.click('#update-settings')

        // update with duplicate data
        .expect(Selector('div')
          .filter('.notification')
          .filter('.is-danger').innerText)
          .eql('BadRequest: The data has already');

    await t
        .typeText('#firstname', goodFirstname, { replace: true })
        .typeText('#lastname', goodLastname, { replace: true })
        .typeText('#secondname', goodSecondname, { replace: true })
        // .typeText('#username', goodUsername, { replace: true })
        .typeText('#email', goodEmail, { replace: true })
        .typeText('#description', goodDescription, { replace: true })
        .typeText('#position', goodPosition, { replace: true })
        .typeText('#phone', goodPhone, { replace: true })
        .typeText('#address', goodAddress, { replace: true })
        .click(programSelect)
        .click(programOption.withText(programTarget2))
        .click(regionSelect)
        .click(regionOption.withText(regionTarget2))
        .click('#update-settings')

        // return old data
        .expect(Selector('span')
          .filter('.tag')
          .filter('.is-info')
          .filter('.is-light')
          .innerText).eql('Updated rows: 6');
});
