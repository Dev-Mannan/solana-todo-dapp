import React from 'react'

export const Header = () => {
  return (
    <div style={{ backgroundColor: '#000', color: '#fff', padding: '1rem', textAlign: 'center' }}>
      <h1 style={{ margin: 0, fontSize: '2rem', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <img src="download.png" alt="Your Image" style={{ width: '50px', height: 'px' }} />
        To Do Dapp
      </h1>
      <h6>Build with Solana</h6>
    </div>
  )
}