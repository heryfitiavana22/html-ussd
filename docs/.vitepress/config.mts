import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "HTML USSD",
  description: "Simulate a USSD experience from HTML in your terminal",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Examples", link: "/markdown-examples" },
    ],

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
        ],
      },
      { text: "How cache works", link: "/how-cache-works" },
      {
        text: "Examples",
        items: [
          { text: "Markdown Examples", link: "/markdown-examples" },
          { text: "Runtime API Examples", link: "/api-examples" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/vuejs/vitepress" },
    ],
  },
});
