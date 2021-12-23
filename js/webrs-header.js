import { LitElement, html, css } from "lit-element/lit-element.js";
import "@material/mwc-top-app-bar-fixed";
import "@material/mwc-icon-button";
import "@material/mwc-button";

class WebRsHeader extends LitElement {
  static get styles() {
    return [
      css`
        a {
          text-decoration: none;
        }
        [slot="title"] {
          font-family: "Roboto Mono", monospace;
          -webkit-font-smoothing: antialiased;
          font-size: 1.25rem;
          line-height: 2rem;
          letter-spacing: 0.4px;
        }
        .white {
          --mdc-theme-primary: white;
          --mdc-button-outline-color: white;
          color: white;
        }
      `,
    ];
  }

  static get properties() {
    return {
      component: { type: String },
      package: { type: String },
    };
  }

  render() {
    let pkg = this.package || "";

    if (pkg.length && pkg[pkg.length - 1]) {
      pkg = pkg + "/";
    }

    return html`
      <mwc-top-app-bar-fixed>
        <a href="../" slot="navigationIcon">
          <mwc-icon-button class="white" icon="arrow_back"></mwc-icon-button>
        </a>
        <span slot="title">${this.component}</span>
        <!-- <a href="/${pkg}index.html" slot="actionItems">
          <mwc-button outlined label="View Source" class="white"></mwc-button>
        </a> -->
      </mwc-top-app-bar-fixed>
    `;
  }
}

customElements.define("webrs-header", WebRsHeader);
