// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> About</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item expanded "><a href="configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="crds.html"><strong aria-hidden="true">4.</strong> Custom Resource Definitions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="crds/keycloakinstance.html"><strong aria-hidden="true">4.1.</strong> KeycloakInstance</a></li><li class="chapter-item expanded "><a href="crds/keycloakapiobject.html"><strong aria-hidden="true">4.2.</strong> KeycloakApiObject</a></li><li class="chapter-item expanded "><a href="crds/keycloakauthenticationflow.html"><strong aria-hidden="true">4.3.</strong> KeycloakAuthenticationFlow</a></li><li class="chapter-item expanded "><a href="crds/keycloakauthenticatorconfig.html"><strong aria-hidden="true">4.4.</strong> KeycloakAuthenticatorConfig</a></li><li class="chapter-item expanded "><a href="crds/keycloakclient.html"><strong aria-hidden="true">4.5.</strong> KeycloakClient</a></li><li class="chapter-item expanded "><a href="crds/keycloakclientscope.html"><strong aria-hidden="true">4.6.</strong> KeycloakClientScope</a></li><li class="chapter-item expanded "><a href="crds/keycloakcomponent.html"><strong aria-hidden="true">4.7.</strong> KeycloakComponent</a></li><li class="chapter-item expanded "><a href="crds/keycloakgroup.html"><strong aria-hidden="true">4.8.</strong> KeycloakGroup</a></li><li class="chapter-item expanded "><a href="crds/keycloakidentityprovider.html"><strong aria-hidden="true">4.9.</strong> KeycloakIdentityProvider</a></li><li class="chapter-item expanded "><a href="crds/keycloakidentityprovidermapper.html"><strong aria-hidden="true">4.10.</strong> KeycloakIdentityProviderMapper</a></li><li class="chapter-item expanded "><a href="crds/keycloakorganization.html"><strong aria-hidden="true">4.11.</strong> KeycloakOrganization</a></li><li class="chapter-item expanded "><a href="crds/keycloakprotocolmapper.html"><strong aria-hidden="true">4.12.</strong> KeycloakProtocolMapper</a></li><li class="chapter-item expanded "><a href="crds/keycloakrealm.html"><strong aria-hidden="true">4.13.</strong> KeycloakRealm</a></li><li class="chapter-item expanded "><a href="crds/keycloakrequiredactionprovider.html"><strong aria-hidden="true">4.14.</strong> KeycloakRequiredActionProvider</a></li><li class="chapter-item expanded "><a href="crds/keycloakresource.html"><strong aria-hidden="true">4.15.</strong> KeycloakResource</a></li><li class="chapter-item expanded "><a href="crds/keycloakrole.html"><strong aria-hidden="true">4.16.</strong> KeycloakRole</a></li><li class="chapter-item expanded "><a href="crds/keycloakscope.html"><strong aria-hidden="true">4.17.</strong> KeycloakScope</a></li><li class="chapter-item expanded "><a href="crds/keycloakuser.html"><strong aria-hidden="true">4.18.</strong> KeycloakUser</a></li></ol></li><li class="chapter-item expanded "><a href="architecture.html"><strong aria-hidden="true">5.</strong> Architecture</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
