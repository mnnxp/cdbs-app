import { Selector } from 'testcafe'; // first import testcafe selectors

const apiPort = process.env.PORT || 8080;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

const settingButton = Selector('a').withAttribute('href', '#/settings');

const goodFirstname = "qweasd";
const goodLastname = "qweasd";
const goodSecondname = "qweasd";
const goodUsername = "qwe";
const goodPassword = "qweqwe";
const goodEmail = "qweasd@qwe.qwe";
const goodDescription = "description for qwe";
const goodPosition = "qweasd";
const goodPhone = "qweasd";
const goodAddress = "qweasd";
const goodProgram = "Creo";
const goodRegion = "Altai Republic";



// fixture `Check update user data`// declare the fixture
//     .page `${url}/#/settings`;  // specify the start page
//
// // Unauthorized not token
// test('Token not found', async t => {
//     await t
//         .typeText('#firstname', goodFirstname)
//         .typeText('#lastname', goodLastname)
//         .typeText('#secondname', goodSecondname)
//         .typeText('#username', goodUsername)
//         .typeText('#email', goodEmail)
//         .typeText('#position', goodPosition)
//         .typeText('#phone', goodPhone)
//         .typeText('#address', goodAddress)
//         .click('#update-settings')
//
//         // Use the assertion to check if the actual header text is equal to the expected one
//         .expect(Selector('div')
//           .filter('.notification')
//           .filter('.is-danger').innerText)
//           .eql('BadRequest: Token not found.');
// });

fixture `Check update user data`// declare the fixture
    .page `${url}/#/login`;  // specify the start page

// Update user data and return old data
test('Update user data', async t => {
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
        .typeText('#position', goodPosition)
        .typeText('#phone', goodPhone)
        .typeText('#address', goodAddress)
        .click('#update-settings')

        // check count updated rows
        .expect(Selector('span')
          .filter('.tag')
          .filter('.is-info')
          .filter('.is-light')
          .innerText).eql('Updated rows: 7');

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
        .typeText('#position', goodPosition, { replace: true })
        .typeText('#phone', goodPhone, { replace: true })
        .typeText('#address', goodAddress, { replace: true })
        .click('#update-settings')

        // return old data
        .expect(Selector('span')
          .filter('.tag')
          .filter('.is-info')
          .filter('.is-light')
          .innerText).eql('Updated rows: 7');
});
