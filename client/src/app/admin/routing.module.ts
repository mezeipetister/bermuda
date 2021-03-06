import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { LayoutComponent } from './layout/layout.component';
import { AuthGuard } from '../guard/auth.guard';
import { ProfileComponent } from './profile/profile.component';
import { EmptyComponent } from '../layout/empty/empty.component';
import { UserComponent } from './user/user.component';
import { UserNewComponent } from './user/user-new/user-new.component';
import { UserDetailComponent } from './user/user-detail/user-detail.component';
import { FolderComponent } from './folder/folder.component';
import { FolderNewComponent } from './folder/folder-new/folder-new.component';
import { FolderDetailComponent } from './folder/folder-detail/folder-detail.component';
import { FolderEditComponent } from './folder/folder-edit/folder-edit.component';
import { DocumentNewComponent } from './document/document-new/document-new.component';
import { DocumentDetailComponent } from './document/document-detail/document-detail.component';

const routes: Routes = [
  {
    path: '', component: LayoutComponent, canActivateChild: [AuthGuard], children: [
      {
        path: '', redirectTo: '/folder',
        pathMatch: 'full'
      },
      { path: 'profile', component: ProfileComponent },
      {
        path: 'user', component: EmptyComponent, children: [
          { path: '', component: UserComponent },
          { path: 'new', component: UserNewComponent },
          { path: ':id', component: UserDetailComponent }
        ]
      },
      {
        path: 'folder', component: EmptyComponent, children: [
          { path: '', component: FolderComponent },
          { path: 'new', component: FolderNewComponent },
          { path: ':id', component: FolderDetailComponent },
          { path: ':id/edit', component: FolderEditComponent },
          { path: ':id/new', component: DocumentNewComponent }
        ]
      },
      {
        path: 'document', component: EmptyComponent, children: [
          // { path: '', component: FolderComponent },
          // { path: 'new', component: FolderNewComponent },
          { path: ':id', component: DocumentDetailComponent },
          // { path: ':id/edit', component: FolderEditComponent },
          // { path: ':id/new', component: DocumentNewComponent }
        ]
      },
    ]
  },

];

@NgModule({
  imports: [
    RouterModule.forChild(routes)
  ],
  exports: [
    RouterModule
  ]
})
export class RoutingModule { }
