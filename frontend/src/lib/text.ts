export const makeGraphemeSegmented = (text: string, locale = 'ja') => {
  return new Intl.Segmenter(locale, { granularity: 'grapheme' });
};

export const countGrapheme = (text: string, locale = 'ja') => {
  const segmenter = makeGraphemeSegmented(text, locale);
  return [...segmenter.segment(text)].length;
};