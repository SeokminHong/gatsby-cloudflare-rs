const React = require('react');
const { PageProvider } = require('./src/contexts/PageContext');

exports.wrapRootElement = ({ element, props }) => (
  <PageProvider {...props}>{element}</PageProvider>
);
