import { Selector } from 'testcafe'; // first import testcafe selectors

const apiPort = process.env.PORT || 8080;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

const baseUsername = "asd";
const basePassword = "asd";

fixture `Check login`// declare the fixture
    .page `${url}/#/login`;  // specify the start page

// authentication with bad username
test('Unauthorized', async t => {
    await t
        .typeText('#username', 'badusername')
        .typeText('#password', 'qweqwe')
        .click('#submit-button')

        // Use the assertion to check if the actual header text is equal to the expected one
        .expect(Selector('div')
          .filter('.notification')
          .filter('.is-danger').innerText)
          .eql('Unauthorized');
});

// authentication with correct username/password
test('Ok authentication', async t => {
    await t
        .typeText('#username', baseUsername)
        .typeText('#password', basePassword)
        .click('#submit-button')

        // Use the assertion to check if the actual header text is equal to the expected one
        .expect(Selector('h2').filter('.subtitle').innerText).eql('engineer component supplier');
});
