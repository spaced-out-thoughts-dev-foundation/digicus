import React, { useState, useEffect } from 'react';

const EditableTitle = ({ initial_title, handleChangeTitle, isCallOnThing }) => {
  const [title, setTitle] = useState(initial_title);
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    setTitle(initial_title);
  }, [initial_title]);

  const handleTitleClick = () => {
    setIsEditing(true);
  };

  const handleInputChange = (e) => {
    const newTitle = e.target.value;
    const oldTitle = title;

    handleChangeTitle(newTitle, oldTitle);
    setTitle(newTitle);
  };

  const handleInputBlur = () => {
    setIsEditing(false);
  };

  const handleInputKeyPress = (e) => {
    if (e.key === 'Enter') {
      setIsEditing(false);
    }
  };

  return (
    <div
      style={{
        // marginLeft: '0.5em',
        // marginRight: '0.5em',
        // marginTop: '0.1em',
        // marginBottom: '0.1em',
        padding: '0',
        backgroundColor: isCallOnThing ? 'gray' : 'white',
        margin: '0',
        width: '100%',
        alignItems: 'center',
        alignContent: 'center',
        height: '100%',
        justifyContent: 'center'
      }}
    >
      {isEditing ? (
        <input
          type="text"
          value={title}
          onChange={handleInputChange}
          onBlur={handleInputBlur}
          onKeyPress={handleInputKeyPress}
          autoFocus
        />
      ) : (
        <h1 onClick={handleTitleClick} style={{
          // marginLeft: '0.5em',
          // marginRight: '0.5em',
          // marginTop: '0.1em',
          // marginBottom: '0.1em',
          margin: '0',
          width: '100%',
          padding: '0',
          backgroundColor: isCallOnThing ? 'gray' : 'white',
          height: '100%',

          alignItems: 'center',
          alignContent: 'center',
          justifyContent: 'center'
        }} >{title}</h1>
      )}
    </div>
  );
};

export default EditableTitle;
