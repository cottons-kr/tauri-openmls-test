import { globalStyle } from '@vanilla-extract/css';

globalStyle('html, body, #root', {
  width: '100%',
})

globalStyle('*', {
  margin: 0,
  padding: 0,
  boxSizing: 'border-box',
})

globalStyle('main', {
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'center',
  width: '100%',
  height: '100%',
  backgroundColor: '#282C35',
  paddingTop: 'env(safe-area-inset-top)',
})

globalStyle('p', {
  color: '#FFFFFF',
})
