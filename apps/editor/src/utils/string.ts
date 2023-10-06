export const StringUtils = {
    PUNCTION_REGEX: /[ .,/#!$%^&*;:{}=\-_`~()]/g,
    slugify(str: string): string {
        return str.replaceAll(this.PUNCTION_REGEX, '-').toLowerCase();
    },
};
