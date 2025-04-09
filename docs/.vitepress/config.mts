import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "HTML USSD",
  description: "Simulate a USSD experience from HTML in your terminal",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [{ text: "Home", link: "/" }],

    sidebar: [
      {
        text: "Introduction",
        items: [
          { text: "What is HTML-USSD?", link: "/what-is-html-ussd" },
          { text: "Getting started", link: "/getting-started" },
        ],
      },
      {
        text: "Usage",
        items: [
          {
            text: "HTML Rules",
            collapsed: true,
            items: [
              { text: "General", link: "/html-rules" },
              { text: "html", link: "/tags/html-tag" },
              { text: "head", link: "/tags/head-tag" },
              { text: "title", link: "/tags/title-tag" },
              { text: "body", link: "/tags/body-tag" },
              { text: "p", link: "/tags/p-tag" },
              { text: "a", link: "/tags/a-tag" },
              { text: "form", link: "/tags/form-tag" },
              { text: "input", link: "/tags/input-tag" },
            ],
          },
          { text: "CLI", link: "/cli" },
          { text: "Navigation Behavior", link: "/navigation-behavior" },
        ],
      },
      {
        text: "Advanced",
        items: [
          { text: "How Cache Works", link: "/how-cache-works" },
          { text: "Creating a Custom Renderer", link: "/custom-renderer" },
          {
            text: "Supporting Custom Input Formats",
            link: "/supporting-custom-input-formats",
          },
          { text: "Security and External Requests", link: "/security" },
          { text: "Validator and Transformer", link: "/validator-transformer" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/heryfitiavana22/html-ussd" },
    ],

    search: {
      provider: 'local'
    }
  },
});
