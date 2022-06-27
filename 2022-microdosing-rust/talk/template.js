function calibrationSlide() {
    return `
        <section class="calibration slide">
            <img src="/mnt/common/images/tv-test-pattern.svg"/>
        </section>
    `;
}

const getImageCanvas = (node) => {
    const images = node.findBy({
        context: 'image',
        role: 'canvas',
    });

    if (images && images.length > 0) {
        return images[0];
    }
};

const sectionInlineStyle = (node) => {
    const image = getImageCanvas(node);

    if (image) {
        const roles = node.getRoles();

        let backgroundSize;

        if (roles && roles.includes('contain')) {
            backgroundSize = 'contain';
        } else {
            backgroundSize = 'cover';
        }

        return `style="background-image: url(${node.getImageUri(image.getAttribute('target'))}); background-size: ${backgroundSize}; background-repeat: no-repeat"`;
    }
};

const sectionTitle = (node) => {
    const titleSeparator = node.getDocument().getAttribute('title-separator') || ':';
    const parts = node.getTitle().split(titleSeparator);
    const main = parts[0];
    const subtitle = parts[1];

    if (subtitle) {
        return`
            <header>
                <h2>${main}</h2>
                <h3>${subtitle}</h3>
            </header>
        `;
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
        <section
            class="${sectionRoles(node).join(' ')} ${node.getTitle() === '!' ? 'no-title' : ''}"
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
            <link rel="stylesheet" href="/mnt/common/theme/css/highlight.css">
            <link rel="stylesheet" href="/mnt/common/theme/css/talk.css">
            <script src="/mnt/common/theme/js/highlight.js"></script>
            <script>
                hljs.initHighlightingOnLoad();
            </script>
        </head>
        <body>
            ${calibrationSlide()}
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
