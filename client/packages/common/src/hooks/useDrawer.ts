import create from 'zustand';
import LocalStorage from '../localStorage/LocalStorage';

type DrawerController = {
  hoverOpen: boolean;
  isOpen: boolean;
  hasUserSet: boolean;
  clickedNavPath?: string;
  open: () => void;
  close: () => void;
  toggle: () => void;
  setHoverOpen: (open: boolean) => void;
  setClickedNavPath: (clicked?: string) => void;
};

export const useDrawer = create<DrawerController>(set => {
  const initialValue = LocalStorage.getItem('/appdrawer/open');
  return {
    hasUserSet: initialValue !== null,
    isOpen: !!initialValue,
    hoverOpen: false,
    setClickedNavPath: (clickedNavPath?: string) =>
      set(state => ({ ...state, clickedNavPath })),
    setHoverOpen: hoverOpen => set(state => ({ ...state, hoverOpen })),
    open: () => set(state => ({ ...state, isOpen: true })),
    close: () => set(state => ({ ...state, isOpen: false, hoverOpen: false })),
    toggle: () =>
      set(state => ({ ...state, isOpen: !state.isOpen, hasUserSet: true })),
  };
});

useDrawer.subscribe(({ hasUserSet, isOpen }) => {
  if (hasUserSet) LocalStorage.setItem('/appdrawer/open', isOpen);
});

LocalStorage.addListener<boolean>((key, value) => {
  if (key === '/appdrawer/open') {
    useDrawer.setState(state => ({ ...state, isOpen: value }));
  }
});
