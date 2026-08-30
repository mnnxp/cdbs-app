#[allow(dead_code)]
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum LocaleKey {
    /// 0
    Empty,
    /// `engineer` (was: 1)
    Engineer,
    /// `component` (was: 2)
    Component,
    /// `supplier` (was: 3)
    Supplier,
    /// `Information about 3D models, drawings, standards, suppliers` (was: 4)
    Info3D,
    /// `Automation of access to the platform via API and integration with CAD systems` (was: 5)
    Automation,
    /// `Publishing Vendor and Custom Component Catalogs` (was: 6)
    Catalogs,
    /// `Exchange of information about components, standards and manufacturers` (was: 7)
    ExchangeInfo,
    /// `English` (was: 8)
    English,
    /// `Russian` (was: 9)
    Russian,
    /// `Terms` (was: 10)
    Terms,
    /// `What is?` (was: 11)
    WhatIs,
    /// `API reference` (was: 12)
    ApiReference,
    /// `Sign in` (was: 13)
    SignIn,
    /// `Sign up` (was: 14)
    SignUp,
    /// `Profile` (was: 15)
    Profile,
    /// `Settings` (was: 16)
    Settings,
    /// `Logout` (was: 17)
    Logout,
    /// `Need an account?` (was: 18)
    NeedAccount,
    /// `Username` (was: 19)
    Username,
    /// `Password` (was: 20)
    Password,
    /// `Have an account?` (was: 21)
    HaveAccount,
    /// `Email` (was: 22)
    Email,
    /// `Firstname` (was: 23)
    Firstname,
    /// `Lastname` (was: 24)
    Lastname,
    /// `Secondname` (was: 25)
    Secondname,
    /// `Program` (was: 26)
    Program,
    /// `Region` (was: 27)
    Region,
    /// `by registering account you accept this agreement` (was: 28)
    AcceptTerms,
    /// `terms and conditions` (was: 29)
    TermsAndConditions,
    /// `Updated at: ` (was: 30)
    UpdatedAt,
    /// `Followers: ` (was: 31)
    Followers,
    /// `CERTIFICATES` (was: 32)
    Certificates,
    /// `COMPONENTS all` (was: 33)
    AllComponents,
    /// `fav COMPONENTS` (was: 34)
    FavComponents,
    /// `COMPANIES all` (was: 35)
    AllCompanies,
    /// `fav COMPANIES` (was: 36)
    FavCompanies,
    /// `fav STANDARDS` (was: 37)
    FavStandards,
    /// `fav USERS` (was: 38)
    FavUsers,
    /// ` Position: ` (was: 39)
    PositionLabel,
    /// ` Region: ` (was: 40)
    RegionLabel,
    /// ` Working software: ` (was: 41)
    WorkingSoftware,
    /// `Hide info` (was: 42)
    HideInfo,
    /// `Show info` (was: 43)
    ShowInfo,
    /// `Login` (was: 44)
    Login,
    /// `Create` (was: 45)
    Create,
    /// `Save` (was: 46)
    Save,
    /// `Delete profile data` (was: 47)
    DeleteProfileData,
    /// `Old password` (was: 48)
    OldPassword,
    /// `New password` (was: 49)
    NewPassword,
    /// `Username` (was: 50)
    UsernameLabel,
    /// `Company type` (was: 51)
    CompanyType,
    /// `Firstname` (was: 52)
    FirstnameLabel,
    /// `Lastname` (was: 53)
    LastnameLabel,
    /// `Secondname` (was: 54)
    SecondnameLabel,
    /// `Position` (was: 55)
    Position,
    /// `Phone` (was: 56)
    Phone,
    /// `Address` (was: 57)
    Address,
    /// `Type access` (was: 58)
    TypeAccess,
    /// `Change` (was: 59)
    Change,
    /// `Managing modifications of the component` (was: 60)
    ManagingModifications,
    /// `Description` (was: 61)
    Description,
    /// `Input your password for confirm delete profile` (was: 62)
    ConfirmDeleteProfile,
    /// `Profile` (was: 63)
    ProfileLabel,
    /// `Certificates` (was: 64)
    CertificatesLabel,
    /// `Access` (was: 65)
    Access,
    /// `Site` (was: 66)
    Site,
    /// `Remove profile` (was: 67)
    RemoveProfile,
    /// `Updated access` (was: 68)
    UpdatedAccess,
    /// `Updated password` (was: 69)
    UpdatedPassword,
    /// `Profile delete` (was: 70)
    ProfileDelete,
    /// `this removed all data related with profile, it cannot be canceled!` (was: 71)
    ProfileDeleteWarning,
    /// `Updated rows: ` (was: 72)
    UpdatedRows,
    /// `Last updated: ` (was: 73)
    LastUpdated,
    /// `No certificates found` (was: 74)
    NoCertificates,
    /// `no data` (was: 75)
    NoData,
    /// `Open profile` (was: 76)
    OpenProfile,
    /// `Profile` (was: 77)
    ProfileTitle,
    /// `Profile picture` (was: 78)
    ProfilePicture,
    /// `Select a supplier` (was: 79)
    SelectSupplier,
    /// `Access Policy` (was: 80)
    AccessPolicy,
    /// `* please note that login is by username, not email.` (was: 81)
    LoginNote,
    /// `Remove profile` (was: 82)
    RemoveProfileTitle,
    /// `Upload new certificate` (was: 83)
    UploadCertificate,
    /// `It is recommended to upload the certificate in image format.` (was: 84)
    CertificateRecommendation,
    /// `Files selected for upload` (was: 85)
    FilesSelected,
    /// `Certificate file` (was: 86)
    CertificateFile,
    /// `Upload` (was: 87)
    Upload,
    /// `Clear` (was: 88)
    Clear,
    /// `Success` (was: 89)
    Success,
    /// `Certificate uploaded!` (was: 90)
    CertificateUploaded,
    /// `Company logo` (was: 91)
    CompanyLogo,
    /// `Image updated!` (was: 92)
    ImageUpdated,
    /// `Image file` (was: 93)
    ImageFile,
    /// `user uploaded ` (was: 94)
    UserUploaded,
    /// `updated at ` (was: 95)
    UpdatedAtLabel,
    /// `Life cycle stage` (was: 96)
    LifeCycleStage,
    /// `Component type` (was: 97)
    ComponentType,
    /// `See more` (was: 98)
    SeeMore,
    /// `See less` (was: 99)
    SeeLess,
    /// `Modifications` (was: 100)
    Modifications,
    /// `Сharacteristics` (was: 101)
    Characteristics,
    /// `Component Files` (was: 102)
    ComponentFiles,
    /// `Standards` (was: 103)
    Standards,
    /// `Catalogs` (was: 104)
    CatalogsLabel,
    /// `Keywords` (was: 105)
    Keywords,
    /// `Files from the file set of the selected modification` (was: 106)
    FilesFromFileset,
    /// `Suppliers` (was: 107)
    Suppliers,
    /// `Main supplier` (was: 108)
    MainSupplier,
    /// `Company` (was: 109)
    Company,
    /// `Name` (was: 110)
    Name,
    /// `Action` (was: 111)
    Action,
    /// `Classifier` (was: 112)
    Classifier,
    /// `Specified tolerance` (was: 113)
    SpecifiedTolerance,
    /// `Attention!` (was: 114)
    Attention,
    /// `Back` (was: 115)
    Back,
    /// `Basic Info` (was: 116)
    BasicInfo,
    /// `Add` (was: 117)
    Add,
    /// `Created by user` (was: 118)
    CreatedByUser,
    /// `Files of the selected modification` (was: 119)
    SelectedModificationFiles,
    /// `Filename` (was: 120)
    Filename,
    /// `Content` (was: 121)
    Content,
    /// `Filesize` (was: 122)
    Filesize,
    /// `Add a supplier for the component` (was: 123)
    AddSupplierForComponent,
    /// `Upload by` (was: 124)
    UploadBy,
    /// `Upload at` (was: 125)
    UploadAt,
    /// `Download` (was: 126)
    Download,
    /// `Edit` (was: 127)
    Edit,
    /// `Info` (was: 128)
    Info,
    /// `Select` (was: 129)
    Select,
    /// `Adding a parameter and its value` (was: 130)
    AddingParameter,
    /// `Adding a value for the modification parameter` (was: 131)
    AddingModificationParamValue,
    /// `Change the value` (was: 132)
    ChangeValue,
    /// `Set a value` (was: 133)
    SetValue,
    /// `Change value` (was: 134)
    ChangeValueLabel,
    /// `Delete` (was: 135)
    Delete,
    /// `No additional parameters` (was: 136)
    NoAdditionalParams,
    /// `Get link` (was: 137)
    GetLink,
    /// `Files from the file set for ` (was: 138)
    FilesFromFilesetFor,
    /// `Certificate removed!` (was: 139)
    CertificateRemoved,
    /// `Description updated!` (was: 140)
    DescriptionUpdated,
    /// `Owner: ` (was: 141)
    Owner,
    /// `Classifier: ` (was: 142)
    ClassifierLabel,
    /// `Show standard` (was: 143)
    ShowStandard,
    /// `Specified tolerance: ` (was: 144)
    SpecifiedToleranceLabel,
    /// `type access ` (was: 145)
    TypeAccessLabel,
    /// `classifier` (was: 146)
    ClassifierLabel2,
    /// `specified tolerance` (was: 147)
    SpecifiedToleranceLabel2,
    /// `technical committee` (was: 148)
    TechnicalCommittee,
    /// `publication at` (was: 149)
    PublicationAt,
    /// `standard status` (was: 150)
    StandardStatus,
    /// `region` (was: 151)
    RegionLabel2,
    /// `Сharacteristics of the standard` (was: 152)
    CharacteristicsOfStandard,
    /// `Files of the standard` (was: 153)
    FilesOfStandard,
    /// `Components` (was: 154)
    ComponentsLabel,
    /// `Publication at` (was: 155)
    PublicationAtLabel,
    /// `Updated at` (was: 156)
    UpdatedAtLabel2,
    /// `Managing master data of the standard` (was: 157)
    ManagingStandardMasterData,
    /// `Supplier` (was: 158)
    SupplierLabel,
    /// `LCS` (was: 159)
    LCS,
    /// `manufactured by ` (was: 160)
    ManufacturedBy,
    /// `Open` (was: 161)
    Open,
    /// `License` (was: 162)
    License,
    /// `Reg.№ ` (was: 163)
    RegNumber,
    /// `Region: ` (was: 164)
    RegionLabel3,
    /// `Show company` (was: 165)
    ShowCompany,
    /// `Add supplier for component` (was: 166)
    AddSupplierForComponentLabel,
    /// `Set owner supplier` (was: 167)
    SetOwnerSupplier,
    /// `Select supplier` (was: 168)
    SelectSupplierLabel,
    /// `Supplier description` (was: 169)
    SupplierDescription,
    /// `Orgname` (was: 170)
    Orgname,
    /// `Shortname` (was: 171)
    Shortname,
    /// `Modification Files` (was: 172)
    ModificationFiles,
    /// `Filesets` (was: 173)
    Filesets,
    /// `Add new modification` (was: 174)
    AddNewModification,
    /// `Create new modification` (was: 175)
    CreateNewModification,
    /// `Modification name` (was: 176)
    ModificationName,
    /// `Modification Data` (was: 177)
    ModificationData,
    /// `Parameter` (was: 178)
    Parameter,
    /// `Value` (was: 179)
    Value,
    /// `Add parameter` (was: 180)
    AddParameter,
    /// `Adding a parameter to the component` (was: 181)
    AddingParameterToComponent,
    /// `Select preview image` (was: 182)
    SelectPreviewImage,
    /// `Possible format` (was: 183)
    PossibleFormat,
    /// `Update Preview` (was: 184)
    UpdatePreview,
    /// `Manage component characteristics` (was: 185)
    ManageComponentCharacteristics,
    /// `Upload component files` (was: 186)
    UploadComponentFiles,
    /// `Manage component files` (was: 187)
    ManageComponentFiles,
    /// `Files for component` (was: 188)
    FilesForComponent,
    /// `Manage component standards` (was: 189)
    ManageComponentStandards,
    /// `Manage component suppliers` (was: 190)
    ManageComponentSuppliers,
    /// `Add standard for component` (was: 191)
    AddStandardForComponent,
    /// `Enter data for catalogs search` (was: 192)
    EnterDataForCatalogsSearch,
    /// `Enter keywords separated by spaces or commas` (was: 193)
    EnterKeywords,
    /// `No file uploaded` (was: 194)
    NoFileUploaded,
    /// `Select files for fileset` (was: 195)
    SelectFilesForFileset,
    /// `Add fileset` (was: 196)
    AddFileset,
    /// `Upload files for fileset` (was: 197)
    UploadFilesForFileset,
    /// `Files of fileset` (was: 198)
    FilesOfFileset,
    /// `Open component` (was: 199)
    OpenComponent,
    /// `Select files for component` (was: 200)
    SelectFilesForComponent,
    /// `Select files for modification` (was: 201)
    SelectFilesForModification,
    /// `Upload modification files` (was: 202)
    UploadModificationFiles,
    /// `Files for modification` (was: 203)
    FilesForModification,
    /// `Files not found` (was: 204)
    FilesNotFound,
    /// `Set a paramname (letter case has matter)` (was: 205)
    SetParamname,
    /// `Adding a set of files` (was: 206)
    AddingFileset,
    /// `Select a set of files` (was: 207)
    SelectFilesetLabel,
    /// `Paste the copied table data here.` (was: 208)
    PasteTableData,
    /// `Importing component parameters` (was: 209)
    ImportingComponentParams,
    /// `Name of parameter` (was: 210)
    NameOfParameter,
    /// `Changing the parameter value` (was: 211)
    ChangingParameterValue,
    /// `Select standard` (was: 212)
    SelectStandard,
    /// `Data updated! Change rows:` (was: 213)
    DataUpdatedChangeRows,
    /// `Data updated` (was: 214)
    DataUpdated,
    /// `Change represent` (was: 215)
    ChangeRepresent,
    /// `Representation type` (was: 216)
    RepresentationType,
    /// `Delete component` (was: 217)
    DeleteComponent,
    /// `For confirm deleted all data this ` (was: 218)
    ConfirmDeleteComponent1,
    /// ` component enter this uuid:` (was: 219)
    ConfirmDeleteComponent2,
    /// `Yes, delete` (was: 220)
    YesDelete,
    /// `Cancel` (was: 221)
    Cancel,
    /// `Choose files for standard…` (was: 222)
    ChooseFilesForStandard,
    /// `Owner company` (was: 223)
    OwnerCompany,
    /// `Manage characteristics of the standard` (was: 224)
    ManageStandardCharacteristics,
    /// `Files of the standard` (was: 225)
    FilesOfStandardLabel,
    /// `Open standard` (was: 226)
    OpenStandard,
    /// `Delete standard` (was: 227)
    DeleteStandard,
    /// ` standard enter this uuid:` (was: 228)
    ConfirmDeleteStandard,
    /// `Set characteristics for the standard` (was: 229)
    SetStandardCharacteristics,
    /// `New representative` (was: 230)
    NewRepresentative,
    /// `Here since` (was: 231)
    HereSince,
    /// `Information` (was: 232)
    Information,
    /// `Type` (was: 233)
    Type,
    /// `To upload data, simply copy the data from a spreadsheet (Excel, LibreOffice, etc.) and paste it into the field below.` (was: 234)
    UploadDataInstructions,
    /// `To add and change the component parameters, you need to insert data into two columns. The first column contains the names of the parameters, and the second column contains their values.<br/>The values specified for the parameters must be different from the existing values.` (was: 235)
    ImportParamsInstructions,
    /// `Filename` (was: 236)
    FilenameLabel,
    /// `Content type` (was: 237)
    ContentType,
    /// `Filesize` (was: 238)
    FilesizeLabel,
    /// `Program` (was: 239)
    ProgramLabel,
    /// `Upload by` (was: 240)
    UploadByLabel,
    /// `Upload at` (was: 241)
    UploadAtLabel,
    /// `Created at` (was: 242)
    CreatedAt,
    /// `* keywords must be less than 100 characters` (was: 243)
    KeywordsLimit,
    /// `Add a license for a component` (was: 244)
    AddLicense,
    /// `Select a license` (was: 245)
    SelectLicense,
    /// `ID:` (was: 246)
    ID,
    /// `Patch:` (was: 247)
    Patch,
    /// `Error loading model` (was: 248)
    ErrorLoadingModel,
    /// `Unsupported format` (was: 249)
    UnsupportedFormat,
    /// `Original Textures` (was: 250)
    OriginalTextures,
    /// `THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.` (was: 251)
    SoftwareLicense,
    /// `Controls` (was: 252)
    Controls,
    /// `Material` (was: 253)
    Material,
    /// `Making The Present Better` (was: 254)
    MakingPresentBetter,
    /// `Lighting` (was: 255)
    Lighting,
    /// `Overviews` (was: 256)
    Overviews,
    /// `Version ` (was: 257)
    Version,
    /// `© CADBase Platform` (was: 258)
    Copyright,
    /// `Glossary` (was: 259)
    Glossary,
    /// `Model Info` (was: 260)
    ModelInfo,
    /// `Show profile` (was: 261)
    ShowProfileLabel,
    /// `Filename: ` (was: 262)
    FilenameLabel2,
    /// `Adding a standard to the component` (was: 263)
    AddingStandardToComponent,
    /// `Update Company` (was: 264)
    UpdateCompany,
    /// `Open company` (was: 265)
    OpenCompanyLabel,
    /// `Representations` (was: 266)
    Representations,
    /// `Remove company` (was: 267)
    RemoveCompany,
    /// `Delete company` (was: 268)
    DeleteCompanyLabel,
    /// `Privacy Notice` (was: 269)
    PrivacyNotice,
    /// `Company don't have representations` (was: 270)
    NoRepresentations,
    /// `Update access` (was: 271)
    UpdateAccess,
    /// `Warning: ` (was: 272)
    Warning,
    /// `All company-related data will be permanently deleted with no possibility of recovery!` (was: 273)
    CompanyDeleteWarning,
    /// `Company delete` (was: 274)
    CompanyDelete,
    /// ` supplier` (was: 275)
    SupplierLabel2,
    /// `Created at` (was: 276)
    CreatedAtLabel,
    /// `Updated at` (was: 277)
    UpdatedAtLabel3,
    /// ` Email: ` (was: 278)
    EmailLabel,
    /// ` Phone: ` (was: 279)
    PhoneLabel,
    /// ` Reg.№ ` (was: 280)
    RegNumberLabel,
    /// ` Location: ` (was: 281)
    Location,
    /// ` Site: ` (was: 282)
    SiteLabel,
    /// `Sphere of activity` (was: 283)
    SphereOfActivity,
    /// `Notifications` (was: 284)
    Notifications,
    /// `CADBase conditions` (was: 285)
    CADBaseConditions,
    /// `Members` (was: 286)
    Members,
    /// `If you need support or help, please contact us: ` (was: 287)
    SupportContact,
    /// `Great!` (was: 288)
    Great,
    /// `Create company` (was: 289)
    CreateCompany,
    /// `Create component` (was: 290)
    CreateComponent,
    /// `Create standard` (was: 291)
    CreateStandard,
    /// `Representative removed!` (was: 292)
    RepresentativeRemoved,
    /// `Representative created!` (was: 293)
    RepresentativeCreated,
    /// `No child component available` (was: 294)
    NoChildComponent,
    /// `Hide components` (was: 295)
    HideComponents,
    /// `See components` (was: 296)
    SeeComponents,
    /// `No file to display (search is performed in the selected fileset)` (was: 297)
    NoFileToDisplay,
    /// `Expand` (was: 298)
    Expand,
    /// `Collapse` (was: 299)
    Collapse,
    /// `View` (was: 300)
    View,
    /// `Close` (was: 301)
    Close,
    /// `Coordinate axes` (was: 302)
    CoordinateAxes,
    /// `Rotation` (was: 303)
    Rotation,
    /// `Frame` (was: 304)
    Frame,
    /// `Model color` (was: 305)
    ModelColor,
    /// `Background color` (was: 306)
    BackgroundColor,
    /// `Model Scale` (was: 307)
    ModelScale,
    /// `Revision` (was: 308)
    Revision,
    /// `Rev.` (was: 309)
    Rev,
    /// `Size` (was: 310)
    Size,
    /// `Uploaded` (was: 311)
    Uploaded,
    /// `Created` (was: 312)
    Created,
    /// `Loaded` (was: 313)
    Loaded,
    /// `Hide` (was: 314)
    Hide,
    /// `Show` (was: 315)
    Show,
    /// `bytes` (was: 316)
    Bytes,
    /// `KB` (was: 317)
    KB,
    /// `MB` (was: 318)
    MB,
    /// `GB` (was: 319)
    GB,
    /// `TB` (was: 320)
    TB,
    /// `* pay attention to the correct character order, case and input language when entering data.` (was: 321)
    InputWarning,
    /// `Copy` (was: 322)
    Copy,
    /// `Copied` (was: 323)
    Copied,
    /// `No result` (was: 324)
    NoResult,
    /// `Open 3D view` (was: 325)
    Open3DView,
    /// `Add to bookmarks` (was: 326)
    AddToBookmarks,
    /// `Remove from bookmarks` (was: 327)
    RemoveFromBookmarks,
    /// `Share` (was: 328)
    Share,
    /// `Show files of the selected modification` (was: 329)
    ShowSelectedModificationFiles,
    /// `Managing files of the standard` (was: 330)
    ManagingStandardFiles,
    /// `Upload files for the standard` (was: 331)
    UploadFilesForStandard,
    /// `Unauthorized` (was: 332)
    Unauthorized,
    /// `Go to page authorization` (was: 333)
    GoToAuthorization,
    /// `Edit` (was: 334)
    EditLabel,
    /// `Preview` (was: 335)
    Preview,
    /// `Markdown markup language is supported` (was: 336)
    MarkdownSupported,
    /// `For example, the <sup></sup> tag is used to specify superscript characters, and «m<sup>2</sup>» is used to spell square meter.` (was: 337)
    MarkdownExample,
    /// `Message to change` (was: 338)
    MessageToChange,
    /// `Messages will help understand: Why was this change made? What effect did the change have? Why was this change necessary?` (was: 339)
    MessageHelp,
    /// `The maximum message length is 500 characters` (was: 340)
    MaxMessageLength,
    /// `Message` (was: 341)
    Message,
    /// `Import modifications and parameters` (was: 342)
    ImportModifications,
    /// `The first row of the table is used to specify parameter names.<br/>Keywords are used to specify modifications data:<br/><b>[ModificationName]</b> - column with modification names (required),<br/><b>[ModificationDescription]</b> - column with descriptions,<br/><b>[ModificationActualStatusId]</b> - column for state IDs.` (was: 343)
    ImportModificationsInstructions,
    /// `Example:\n[ModificationName]\tparameter 1\tparameter 2\nmodification name 1\tvalue 1\tvalue 2\nmodification name 2\tvalue 3\tvalue 4` (was: 344)
    ImportExample,
    /// `headers` (was: 345)
    Headers,
    /// `rows` (was: 346)
    Rows,
    /// `Import` (was: 347)
    Import,
    /// `Export` (was: 348)
    Export,
    /// `Search` (was: 349)
    Search,
    /// `No results` (was: 350)
    NoResults,
    /// `Enter search text` (was: 351)
    EnterSearchText,
    /// `Component` (was: 352)
    ComponentLabel,
    /// `Data of the selected modification` (was: 353)
    SelectedModificationData,
    /// `Order calculation` (was: 354)
    OrderCalculation,
    /// `Request` (was: 355)
    Request,
    /// `Cost:` (was: 356)
    Cost,
    /// `Laser cutting calculator` (was: 357)
    LaserCuttingCalculator,
    /// `Calculate the cost of cutting before creating an order` (was: 358)
    CalculatorDescription,
    /// `Material thickness (mm)` (was: 359)
    MaterialThickness,
    /// `Material` (was: 360)
    MaterialLabel,
    /// `Total length of cut (m)` (was: 361)
    TotalCutLength,
    /// `Total number of cuts (pcs)` (was: 362)
    TotalCuts,
    /// `Calculate` (was: 363)
    Calculate,
    /// `Create order` (was: 364)
    CreateOrder,
    /// `Steel` (was: 365)
    Steel,
    /// `Stainless steel` (was: 366)
    StainlessSteel,
    /// `Pure aluminum` (was: 367)
    PureAluminum,
    /// `Aluminum alloys (AMg, AMz)` (was: 368)
    AluminumAlloys,
    /// `Duralumin (D16)` (was: 369)
    Duralumin,
    /// `Brass` (was: 370)
    Brass,
    /// `Copper` (was: 371)
    Copper,
    /// `Titanium` (was: 372)
    Titanium,
    /// `Managing master data of the service` (was: 373)
    ManagingServiceMasterData,
    /// `Manage characteristics of the service` (was: 374)
    ManageServiceCharacteristics,
    /// `Managing files of the service` (was: 375)
    ManagingServiceFiles,
    /// `Files of the service` (was: 376)
    ServiceFiles,
    /// `Upload files for the service` (was: 377)
    UploadServiceFiles,
    /// `Open service` (was: 378)
    OpenService,
    /// `Services` (was: 379)
    Services,
    /// `Discussion` (was: 380)
    Discussion,
    /// `Post` (was: 381)
    Post,
    /// `Reply` (was: 382)
    Reply,
    /// `Show answers` (was: 383)
    ShowAnswers,
    /// `Enter a new comment` (was: 384)
    EnterNewComment,
    /// `By parameters` (was: 385)
    ByParameters,
    /// `By catalogs (categories)` (was: 386)
    ByCatalogs,
    /// `By keywords` (was: 387)
    ByKeywords,
    /// `Only favorites` (was: 388)
    OnlyFavorites,
    /// `Refinement filters` (was: 389)
    RefinementFilters,
    /// `Search by ID` (was: 390)
    SearchByID,
    /// `By company` (was: 391)
    ByCompany,
    /// `By standard` (was: 392)
    ByStandard,
    /// `By user` (was: 393)
    ByUser,
    /// `Enter UUID` (was: 394)
    EnterUUID,
    /// `Reset selection` (was: 395)
    ResetSelection,
    /// `New modification` (was: 396)
    NewModification,
    /// `Data not available` (was: 397)
    DataNotAvailable,
    /// `Permission based access for users/groups` (was: 398)
    PermissionBasedAccess,
    /// `Some data is available to users` (was: 399)
    SomeDataAvailable,
    /// `Available to all users` (was: 400)
    AvailableToAll,
    /// `A file set (fileset) is a logical group of files linked to a specific component modification.` (was: 401)
    FilesetDescription,
    /// `For the selected component modification a file set is missing, so file upload is currently unavailable.` (was: 402)
    NoFilesetForModification,
    /// `No filesets` (was: 403)
    NoFilesets,
    /// `Filesets` (was: 404)
    FilesetsLabel,
    /// `Choose a name for the set (for example, “FreeCAD”, “Blender”, “GLTF”, etc.).` (was: 405)
    ChooseFilesetName,
    /// `Save the set – after that you will be able to upload files to it.` (was: 406)
    SaveFileset,
    /// `A component modification is an individual variant or version of a component that captures specific parameters, material, dimensions, configuration, and other product characteristics.` (was: 407)
    ModificationDescription,
    /// `This action will place this object, including all its related data, in a pending deletion state and delete it permanently.` (was: 408)
    DeleteWarning,
    /// `Are you absolutely sure?` (was: 409)
    AreYouSure,
    /// `After this point, ` (was: 410)
    AfterThisPoint,
    /// ` data cannot be recovered.` (was: 411)
    DataCannotBeRecovered,
    /// `This action can lead to data loss. To prevent accidental actions we ask you to confirm your intention.` (was: 412)
    ConfirmIntention,
    /// `Enter the following to confirm:` (was: 413)
    EnterToConfirm,
    /// `SERVER LOCATION` (was: 414)
    ServerLocation,
    /// `🇳🇱 Netherlands` (was: 415)
    Netherlands,
    /// `🇷🇺 Russia` (was: 416)
    Russia,
    /// `🇨🇳 China` (was: 417)
    China,
    /// `🌐 Custom Server` (was: 418)
    CustomServer,
    /// `View` (was: 419)
    ViewLabel,
    /// `Active Layer` (was: 420)
    ActiveLayer,
    /// `Hide Travel Moves` (was: 421)
    HideTravelMoves,
    /// `Display` (was: 422)
    Display,
    /// `Play` (was: 423)
    Play,
    /// `Speed` (was: 424)
    Speed,
    /// `Select` (was: 425)
    SelectLabel,
    /// `Metalness` (was: 426)
    Metalness,
    /// `Roughness` (was: 427)
    Roughness,
    /// `Env Intensity` (was: 428)
    EnvIntensity,
    /// `Clearcoat` (was: 429)
    Clearcoat,
    /// `Clearcoat Rough` (was: 430)
    ClearcoatRough,
    /// `Ambient` (was: 431)
    Ambient,
    /// `Directional` (was: 432)
    Directional,
    /// `Light` (was: 433)
    Light,
    /// `File` (was: 434)
    File,
    /// `Size` (was: 435)
    SizeLabel,
    /// `F: fullscreen | 0-9: views (5: ortho/persp)` (was: 436)
    FullscreenHint,
    /// `Hide Textures` (was: 437)
    HideTextures,
    /// `all` (was: 438)
    All,
    /// `up to current` (was: 439)
    UpToCurrent,
    /// `current only` (was: 440)
    CurrentOnly,
    /// `Perspective (3D)` (was: 441)
    Perspective3D,
    /// `Top (XY)` (was: 442)
    TopXY,
    /// `Bottom` (was: 443)
    Bottom,
    /// `Front (XZ)` (was: 444)
    FrontXZ,
    /// `Back` (was: 445)
    BackView,
    /// `Left (YZ)` (was: 446)
    LeftYZ,
    /// `Right` (was: 447)
    Right,
    /// `Isometric` (was: 448)
    Isometric,
    /// `Drag here or click to select` (was: 449)
    DragOrClick,
    /// `Files to upload` (was: 450)
    FilesToUpload,
    /// `Pending` (was: 451)
    Pending,
    /// `Uploading` (was: 452)
    Uploading,
    /// `Completed` (was: 453)
    Completed,
    /// `Failed` (was: 454)
    Failed,
    /// ` file(s) are in storage and awaiting confirmation` (was: 455)
    FilesInStorage,
    /// ` file(s) failed to upload` (was: 456)
    FilesFailed,
    /// `Upload URL not found` (was: 457)
    UploadUrlNotFound,
    /// `Granted At` (was: 458)
    GrantedAt,
    /// `Users Access` (was: 459)
    UsersAccess,
    /// `Companies Access` (was: 460)
    CompaniesAccess,
    /// `Actions` (was: 461)
    Actions,
    /// `Company Roles & Permissions` (was: 462)
    CompanyRolesPermissions,
    /// `New Role` (was: 463)
    NewRole,
    /// `No Roles Yet` (was: 464)
    NoRolesYet,
    /// `Roles help you organize team members and control their access to company resources.` (was: 465)
    RolesHelp,
    /// `Create First Role For The Company` (was: 466)
    CreateFirstRole,
    /// `Role Name` (was: 467)
    RoleName,
    /// `Access Permissions` (was: 468)
    AccessPermissions,
    /// `No permissions` (was: 469)
    NoPermissions,
    /// `Create Company Role` (was: 470)
    CreateCompanyRole,
    /// `Role` (was: 471)
    Role,
    /// `Type to new role name...` (was: 472)
    TypeNewRoleName,
    /// `Edit Role` (was: 473)
    EditRole,
    /// `Role ID:` (was: 474)
    RoleID,
    /// `Select which permissions this role should have` (was: 475)
    SelectPermissions,
    /// `This role will have no permissions. Members with this role won't be able to do anything.` (was: 476)
    RoleNoPermissions,
    /// `Company Members` (was: 477)
    CompanyMembers,
    /// `Add Member` (was: 478)
    AddMember,
    /// `Manage team members can do access your company.` (was: 479)
    ManageTeamMembers,
    /// `No members in this company.` (was: 480)
    NoMembers,
    /// `Type to search users...` (was: 481)
    SearchUsers,
    /// `Type to search companies...` (was: 482)
    SearchCompanies,
    /// `Joined At` (was: 483)
    JoinedAt,
    /// `Add User` (was: 484)
    AddUser,
    /// `Add User Access` (was: 485)
    AddUserAccess,
    /// `Search User` (was: 486)
    SearchUser,
    /// `Access Level` (was: 487)
    AccessLevel,
    /// `Add Company Access` (was: 488)
    AddCompanyAccess,
    /// `Search Company` (was: 489)
    SearchCompany,
    /// `Add Company Member` (was: 490)
    AddCompanyMember,
    /// `Add Company` (was: 491)
    AddCompany,
    /// `No users found` (was: 492)
    NoUsersFound,
    /// `Found:` (was: 493)
    Found,
    /// `Avatar` (was: 494)
    Avatar,
    /// `No users have access to this component.` (was: 495)
    NoUsersAccess,
    /// `No companies found` (was: 496)
    NoCompaniesFound,
    /// `No companies have access to this component.` (was: 497)
    NoCompaniesAccess,
    /// `Customize roles to control what team members can do within your company.` (was: 498)
    CustomizeRoles,
    /// `Select fileset` (was: 499)
    SelectFilesetLabel2,
    /// `Start typing name...` (was: 500)
    StartTypingName,
    /// `Saving selection...` (was: 501)
    SavingSelection,
    /// `Hemisphere` (was: 502)
    Hemisphere,
    /// `Key Light` (was: 503)
    KeyLight,
    /// `Fill Light` (was: 504)
    FillLight,
    /// `Rim Light` (was: 505)
    RimLight,
    /// `Key Light Position` (was: 506)
    KeyLightPosition,
    /// `Fill Light Position` (was: 507)
    FillLightPosition,
    /// `Rim Light Position` (was: 508)
    RimLightPosition,
    /// `Intensities` (was: 509)
    Intensities,
    /// `Positions` (was: 510)
    Positions,
    _Count,
}