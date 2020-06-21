function calibrationSlide() {
    return `
        <section class="calibration slide">
            <img src="/mnt/common/images/tv-test-pattern.svg"/>
        </section>
    `;
}

function jokeSlide() {
    return `
        <section class="title slide">
            <header>
                <h1>Why I switched from Rust to Go</h1>
                <h2>... and why I'm never going back</h2>
            </header>
            <main>
                <img src="assets/images/gopher.png"/>
            </main>
        </section>
    `;
}

function titleSlideHeader(node) {
    const title = node.getDocumentTitle({
        partition: true,
    });

    if (title.hasSubtitle()) {
        return `
            <h1>${title.getMain()}</h1>
            <h2>${title.getSubtitle()}</h2>
        `;
    } else {
        return `
            <h1>${node.getDocumentTitle()}</h1>
        `;
    }
}

function titleSlide(node) {
    return `
        <section class="title slide">
            <header>
                ${titleSlideHeader(node)}
            </header>
            <footer>
                <div class="logo-wrapper">
                    <img class="logo" src="/mnt/common/images/logos/rust-wroclaw.png"/>
                </div>
                
                <div class="author-wrapper">
                    <p class="author">
                        ${node.getDocument().getAuthor()}
                    </p>
                </div>
            </footer>
        </section>
    `;
}

const getImageCanvas = (node) => {
    const images = node.findBy({
        context: 'image',
        role: 'canvas',
    });

    if (images && images.length > 0) {
        return images[0]
    }
};

const sectionInlineStyle = (node) => {
    const image = getImageCanvas(node);

    if (image) {
        const roles = node.getRoles();

        let backgroundSize;

        if (roles && roles.includes('contain')) {
            backgroundSize = 'contain'
        } else {
            backgroundSize = 'cover'
        }

        return `style="background-image: url(${node.getImageUri(image.getAttribute('target'))}); background-size: ${backgroundSize}; background-repeat: no-repeat"`;
    } else {
        return '';
    }
};

const sectionTitle = (node) => {
    const titleSeparator = node.getDocument().getAttribute('title-separator') || ':';
    const parts = node.getTitle().split(titleSeparator);
    const main = parts[0];
    const subtitle = parts[1];
    if (subtitle) {
        return `<header>
  <h2>${main}</h2>
  <h3>${subtitle}</h3>
</header>`
    }
    return `<h2>${node.getTitle()}</h2>`
};

const sectionRoles = (node) => {
    const roles = node.getRoles() || [];

    roles.unshift('slide');

    const image = getImageCanvas(node);

    if (image) {
        roles.push('image');
    }

    return roles;
};

const elementId = (node) => {
    const id = node.getId();

    if (id) {
        return ` id="${id}"`;
    }

    return '';
};

module.exports = {
    paragraph: (node) => `
        <p class="${node.getRoles().join(' ')}">
            ${node.getContent()}
        </p>
    `,

    section: (node) => `
        <section class="${sectionRoles(node).join(' ')} ${node.getTitle() === '!' ? 'no-title' : ''}"
                 data-slide-number="${node.index + 1}"
                 data-slide-count="${node.parent.blocks.length}"
                 ${sectionInlineStyle(node)}>
            ${sectionTitle(node)}
            ${node.getContent()}
        </section>
    `,

    document: (node) => `
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8"/>
            <link rel="stylesheet" href="/mnt/common/theme/css/asciidoctor.css">
            <link rel="stylesheet" href="/mnt/common/theme/css/highlighting-github.css">
            <link rel="stylesheet" href="/mnt/common/theme/css/talk.css">
            <script src="/mnt/common/theme/js/highlight.js"></script>
            <script>
                hljs.initHighlightingOnLoad();
            </script>
        </head>
        <body>
            ${calibrationSlide()}
            ${jokeSlide()}
            ${titleSlide(node)}
            ${node.getContent()}
        </body>
    `,

    open: (node) => `
        <div${elementId(node)} class="${node.getRoles().join(' ')}">
            ${node.getContent()}
        </div>
    `,

    image: (node) => {
        const roles = node.getRoles();

        if (roles && roles.includes('canvas')) {
            return '';
        }

        const src = node.getImageUri(node.getAttribute('target'));
        const height = node.getAttribute('height');
        const width = node.getAttribute('width');

        return `
            <figure class="image ${node.getRoles().join(' ')}">
                <img src="${src}" height="${height}" width="${width}"/>
            </figure>
        `;
    }
};
