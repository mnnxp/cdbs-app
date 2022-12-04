import { Selector } from 'testcafe'; // first import testcafe selectors

const apiPort = process.env.PORT || 8080;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

const programTarget = 'DesignSpark Mechanical';
const programSelect = Selector('#program');
const programOption = programSelect.find('option');

const regionTarget = 'Talas region';
const regionSelect = Selector('#region');
const regionOption = regionSelect.find('option');

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

const registerButton = Selector('a').withAttribute('href', '/register');
const settingButton = Selector('a').withAttribute('href', '/settings');
const profileButton = Selector('a').withAttribute('href', `/profile/${goodUsername}`);

fixture `Check update user data`// declare the fixture
    .page `${url}`;  // specify the start page

// See profile data
test('Self profile data', async t => {
  // open register page
  await t.click(registerButton)

  await t
      .typeText('#firstname', goodFirstname)
      .typeText('#lastname', goodLastname)
      .typeText('#secondname', goodSecondname)
      .typeText('#username', goodUsername)
      .typeText('#email', goodEmail)
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

    // open profile page
    await t.click(profileButton)

    await t
        .typeText('#title-fl', `${goodFirstname} ${goodLastname}`)
        .typeText('#subtitle-username',  goodUsername)
        // .typeText('#description', goodDescription)
        .typeText('#position', goodPosition)
        .typeText('#region', regionTarget)
        .typeText('#program', programTarget);
});
