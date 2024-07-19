use std::collections::BTreeMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Translation of the fields for English
    pub(super) static ref LOCAL_EN: BTreeMap<usize, &'static str> = BTreeMap::from([
            (1, "engineer"),
            (2, "component"),
            (3, "supplier"),
            (4, "Information about 3D models, drawings, standards, suppliers"),
            (5, "Automation of access to the platform via API and integration with CAD systems"),
            (6, "Publishing Vendor and Custom Component Catalogs"),
            (7, "Exchange of information about components, standards and manufacturers"),
            (8, "English"),
            (9, "Russian"),
            (10, "Terms"),
            (11, "About us"),
            (12, "API reference"),
            (13, "Sign in"),
            (14, "Sign up"),
            (15, "Profile"),
            (16, "Settings"),
            (17, "Logout"),
            (18, "Need an account?"),
            (19, "Username"),
            (20, "Password"),
            (21, "Have an account?"),
            (22, "Email"),
            (23, "Firstname"),
            (24, "Lastname"),
            (25, "Secondname"),
            (26, "Program"),
            (27, "Region"),
            (28, "by registering account you accept this agreement"),
            (29, "terms and conditions"),
            (30, "Updated at: "),
            (31, "Followers: "),
            (32, "CERTIFICATES"),
            (33, "COMPONENTS all"),
            (34, "fav COMPONENTS"),
            (35, "COMPANIES all"),
            (36, "fav COMPANIES"),
            (37, "fav STANDARDS"),
            (38, "fav USERS"),
            (39, " Position: "),
            (40, " Region: "),
            (41, " Working software: "),
            (42, "Hide info"),
            (43, "Show info"),
            (44, "Enter"),
            (45, "Create"),
            (46, "Save"),
            (47, "Delete profile data"),
            (48, "Old password"),
            (49, "New password"),
            (50, "Username"),
            (51, "Company type"),
            (52, "Firstname"),
            (53, "Lastname"),
            (54, "Secondname"),
            (55, "Position"),
            (56, "Phone"),
            (57, "Address"),
            (58, "Type access"),
            (59, "Change"),
            (60, "Manage component modifications"),
            (61, "Description"),
            (62, "Input your password for confirm delete profile"),
            (63, "Profile"),
            (64, "Certificates"),
            (65, "Access"),
            (66, "Site"),
            (67, "Remove profile"),
            (68, "Updated access"),
            (69, "Updated password"),
            (70, "Profile delete"),
            (71, "Warning: this removed all data related with profile, it cannot be canceled!"),
            (72, "Updated rows: "),
            (73, "Last updated: "),
            (74, "Not fount certificates"),
            (75, "not data"),
            (76, "Open profile"),
            (77, "Profile"),
            (78, "Favicon"),
            (79, "Select a supplier"),
            (80, "Access Policy"),
            (81, "INN"),
            (82, "Remove profile"),
            (83, "Upload new certificate"),
            (84, "It is recommended to upload the certificate in image format."),
            (85, "Select file: "),
            (86, "Drop certificate file here"),
            (87, "Upload"),
            (88, "Clear"),
            (89, "Success"),
            (90, "This certificate upload!"),
            (91, "It is recommended to upload the favicon in image format."),
            (92, "This favicon upload!"),
            (93, "Drop favicon file here"),
            (94, "user uploaded "),
            (95, "updated at "),
            (96, "Actual status"),
            (97, "Component type"),
            (98, "See more"),
            (99, "See less"),
            (100, "Modifications"),
            (101, "Сharacteristics of the component"),
            (102, "Component files"),
            (103, "Standards"),
            (104, "Catalogs"),
            (105, "Keywords"),
            (106, "Files of select fileset"),
            (107, "Suppliers"),
            (108, "Main supplier"),
            (109, "Company"),
            (110, "Name"),
            (111, "Action"),
            (112, "Classifier"),
            (113, "Specified tolerance"),
            (114, "Type access "),
            (115, "Action | Files"),
            (116, "Modification"),
            (117, "Add"),
            (118, "User uploaded: "),
            (119, "Modification files"),
            (120, "Filename"),
            (121, "Content"),
            (122, "Filesize"),
            (123, "Add a supplier for the component"),
            (124, "Upload by"),
            (125, "Upload at"),
            (126, "Download"),
            (127, "edit"),
            (128, "info"),
            (129, "select"),
            (130, "Adding a parameter and its value"),
            (131, "Adding a value for the modification parameter"),
            (132, "Change the value"),
            (133, "Set a value"),
            (134, "Change value"),
            (135, "Delete"),
            (136, "No additional parameters"),
            (137, "Get link"),
            (138, "Temp solution for download files"),
            (139, "This certificate removed!"),
            (140, "Description updated!"),
            (141, "Owner: "),
            (142, "Classifier: "),
            (143, "Show standard"),
            (144, "Specified tolerance: "),
            (145, "type access "),
            (146, "classifier"),
            (147, "specified tolerance"),
            (148, "technical committee"),
            (149, "publication at"),
            (150, "standard status"),
            (151, "region"),
            (152, "Сharacteristics of the standard"),
            (153, "Files"),
            (154, "Components"),
            (155, "Publication at"),
            (156, "Updated at"),
            (157, "standard"),
            (158, "Supplier: "),
            (159, "Actual status: "),
            (160, "manufactured by "),
            (161, "Open"),
            (162, "License"),
            (163, "Reg.№ "),
            (164, "Region: "),
            (165, "Show company"),
            (166, "Add supplier for component"),
            (167, "Set owner supplier"),
            (168, "Select supplier"),
            (169, "Supplier description"),
            (170, "Orgname"),
            (171, "Shortname"),
            (172, "Manage modification files"),
            (173, "Manage modification filesets"),
            (174, "Add new modification"),
            (175, "Create new modification"),
            (176, "Modification name"),
            (177, "Change modification data"),
            (178, "Parameter"),
            (179, "Value"),
            (180, "Add parameter"),
            (181, "Adding a parameter to the component"),
            (182, "Drop preview image here"),
            (183, "Possible format"),
            (184, "Update image for preview"),
            (185, "Manage component characteristics"),
            (186, "Upload component files"),
            (187, "Manage component files"),
            (188, "Files for component"),
            (189, "Manage component standards"),
            (190, "Manage component suppliers"),
            (191, "Add standard for component"),
            (192, "Enter data for catalogs search"),
            (193, "Enter keywords separated by spaces or commas"),
            (194, "No file uploaded"),
            (195, "Choose files for fileset…"),
            (196, "Add fileset"),
            (197, "Upload files for fileset"),
            (198, "Files of fileset"),
            (199, "Show component"),
            (200, "Choose component files…"),
            (201, "Choose modification files…"),
            (202, "Upload modification files"),
            (203, "Files for modification"),
            (204, "Files not found"),
            (205, "Set a paramname (letter case has matter)"),
            (206, "Adding a set of files for the selected modification"),
            (207, "Select a set of files"),
            (208, "base"),
            (209, "no base"),
            // (210, "Apply"),
            (211, "Changing the parameter value"),
            (212, "Select standard"),
            (213, "Data updated! Change rows:"),
            (214, "Data updated"),
            (215, "Change represent"),
            (216, "Representation type"),
            (217, "Delete component"),
            (218, "For confirm deleted all data this "),
            (219, " component enter this uuid:"),
            (220, "Yes, delete"),
            (221, "Cancel"),
            (222, "Choose files for standard…"),
            (223, "Owner company "),
            (224, "Manage standard characteristics"),
            (225, "Files stadndard"),
            (226, "Open standard"),
            (227, "Delete standard"),
            (228, " standard enter this uuid:"),
            (229, "Set standard characteristics"),
            (230, "New representative"),
            (231, " region "),
            (232, "address"),
            (233, "type"),
            (234, "phone"),
            (235, "representation type"),
            (236, "Filename"),
            (237, "Content type"),
            (238, "Filesize"),
            (239, "Program"),
            (240, "Upload by"),
            (241, "Upload at"),
            (242, "Created at"),
            (243, "Keywords must be less than 10 symbols"),
            (244, "Add a license for a component"),
            (245, "Select a license"),
            (246, "Id:"),
            (247, "Patch:"),
            (248, "Terms CADBase"),
            (249, "Thank you for using CADBase!"),
            (250, "We're really happy you're here. Please read this Terms of Service agreement carefully before accessing or using CADBase. Because it is such an important contract between us and our users, we need to make it as clear as possible:"),
            (251, "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."),
            (252, "Please let us know if there are any errors or problems, also if you need help."),
            (253, "Contact with us: "),
            (254, "Make For Into Future"),
            (255, "CADBase is a platform for publishing and sharing information about 3D components, models, drawings and documentation from manufacturers and users."),
            (256, "We are also happy to welcome all engineers, architects and just good people who love to share ideas, concepts, experiences with others. Regardless of where you live, if you want to receive and share knowledge, then we will try to help you with this."),
            (257, "Project was founded start development in 2018 by Ivan Nosovsky (some guy from Russia), after some year in project accept participial Yulia Gerasimova (since 2019) and Xia Tianhao (夏添豪, since 2021)."),
            (258, "Launched MVP took place in 2022 and we hope that you like this platform."),
            (259, "We are currently looking for manufacturers who are interested in this platform and are willing to provide quality feedback."),
            (260, "Thank you for using CADBase!"),
            (261, "Show profile"),
            (262, "Filename: "),
            (263, "Adding a standard to the component"),
            (264, "Update Company"),
            (265, "Open company"),
            (266, "Representations"),
            (267, "Remove company"),
            (268, "Delete company"),
            (269, "No catalogs associated with the company"),
            (270, "Company don't have representations"),
            (271, "Update access"),
            (272, "Warning: "),
            (273, "this removed all data related with company, it cannot be canceled!"),
            (274, "Company delete"),
            (275, " supplier"),
            (276, "Created at"),
            (277, "Updated at"),
            (278, " Email: "),
            (279, " Phone: "),
            (280, " Reg.№ "),
            (281, " Location: "),
            (282, " Site: "),
            (283, "Sphere of activity: "),
            (284, "Notifications"),
            (285, "CADBase conditions"),
            (286, "Members"),
            (287, "If you need support or help, please contact us: "),
            (288, "Great!"),
            (289, "Create company"),
            (290, "Create component"),
            (291, "Create standard"),
            (292, "This representative removed!"),
            (293, "This representative created!"),
            (294, "No child component available"),
            (295, "Hide components"),
            (296, "See components"),
            (297, "No file to display"),
            (298, "Expand"),
            (299, "Collapse"),
            (300, "View"),
            (301, "Close"),
            (302, "Coordinate axes"),
            (303, "Rotation"),
            (304, "Frame"),
            (305, "Model color"),
            (306, "Background color"),
            (307, "Model Scale"),
            (308, "Revision"),
            (309, "Rev."),
            (310, "Size"),
            (311, "Uploaded"),
            (312, "Created"),
            (313, "Loaded"),
            (314, "Hide"),
            (315, "Show"),
            (316, "bytes"),
            (317, "KB"),
            (318, "MB"),
            (319, "GB"),
            (320, "TB"),
            (321, "Note: fields are case sensitive"),
            (322, "Copy"),
            (323, "Copyed"),
            (324, "No result"),
            (325, "Open 3D view"),
            (326, "Add to bookmarks"),
            (327, "Remove from bookmarks"),
            (328, "Share"),
            (329, "Show files of the selected modification"),
        ]);
}
