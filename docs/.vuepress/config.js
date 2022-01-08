module.exports = {
  locales: {
    "/": {
      lang: "en-US",
      title: "Shortkut",
      description: "The fast, easy to use terminal shortkut manager!",
    },
  },

  theme: "default-prefers-color-scheme",
  themeConfig: {
    // logo: "/icon.png",
    // the GitHub repo path
    repo: "XtremeDev/shortkut",
    // the label linking to the repo
    repoLabel: "GitHub",
    // // if your docs are not at the root of the repo:
    // docsDir: "docs",
    // defaults to false, set to true to enable
    editLinks: true,
    locales: {
      "/": {
        // text for the language dropdown
        selectText: "Languages",
        // label for this locale in the language dropdown
        label: "English",
        // Custom text for edit link. Defaults to "Edit this page"
        editLinkText: "Edit this page on GitHub",
        // Custom sidebar values
        sidebar: ["/", ["/install/", "Install"]],
      },
    },
  },
};
