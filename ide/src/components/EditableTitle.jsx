import React, { useState, useEffect } from 'react';

const EditableTitle = ({ initial_title, handleChangeTitle }) => {
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
    <div>
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
        <h1 onClick={handleTitleClick}>{title}</h1>
      )}
    </div>
  );
};

export default EditableTitle;
