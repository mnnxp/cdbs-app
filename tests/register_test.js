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

const busyUsername = "usernameeee";

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

// Register new user with busy username
test('Check register user with duplicate username', async t => {
    // open register page
    await t.click(registerButton)

    await t
        .typeText('#firstname', goodFirstname)
        .typeText('#lastname', goodLastname)
        .typeText('#secondname', goodSecondname)
        .typeText('#email', goodEmail)
        .typeText('#username', busyUsername)
        .typeText('#password', goodPassword)
        .click(programSelect)
        .click(programOption.withText(goodProgram))
        .click(regionSelect)
        .click(regionOption.withText(goodRegion))
        .click('#submit-button')

    await t
      .expect(Selector('div')
        .filter('.notification')
        .filter('.is-danger').innerText)
        .eql('BadRequest: Failed create new user');
});

// Register new user and check login with new data
test('Check register user', async t => {
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
});
