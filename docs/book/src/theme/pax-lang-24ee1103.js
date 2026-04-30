hljs.registerLanguage('pax', function(hljs) {
  return {
    name: 'Pax',
    keywords: {
      keyword: 'var int str bool print println if elif else',
      literal: 'true false',
    },
    contains: [
      hljs.QUOTE_STRING_MODE,
      hljs.C_LINE_COMMENT_MODE,
      hljs.C_NUMBER_MODE,
      {
        className: 'function',
        begin: /[a-z_]\w*(?=\()/ 
      }
    ]
  };
});
hljs.initHighlightingOnLoad();